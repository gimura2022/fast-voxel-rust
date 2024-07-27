//! # Fast voxel preprocessor
//! This crate provides functions for preprocessing various files.
//! 
//! ## Usage
//! in order to start working with the preprocessor,
//! you can either use the library for preprocessing files directly in the program,
//! or create a ready-made file using the fast-voxel-preprocessor utility.
//! 
//! #### Library
//! To use the library you need to connect the `fast-voxel-preprocessor` crate
//! and then use either the `ShaderPreprocessor` structure or the `preprocess_dir` function.
//! 
//! #### Utility
//! To use the utility you need to install it, you can do this with the command `cargo install fast-voxel-preprocessor`.
//! then you can write the command fast-voxel-preprocessor where the first argument is the folder with the files and the second is the name of the output file.
//! 
//! ## Syntax
//! This section describes the preprocessor syntax.
//! 
//! First of all, in order for the preprocessor to accept your commands,
//! you need to write a comment like `//!` after which you can write preprocessor commands.
//! 
//! At the moment there are the following commands working as in the C preprocessor:
//! - include
//! - define
//! - undef
//! - ifdef
//! - ifndef
//! - endif
//! 
//! There is also an insert directive that works in tandem with a definition
//! and is needed to replace variable names in the code passed to it with their value.
//! 
//! ### Examples
//! 
//! `main` file
//! ```
//! // including greetings file
//! //! include "greetings"
//! 
//! fn main() {
//!     // logic ways
//!     // this code will be enabled if the GREETING_STR variable is found
//!     //! ifdef GREETING_STR
//! 
//!     // using variable
//!     //! insert "println!(GREETING_STR);"
//! 
//!     //! endif
//!     
//!     // if the variable is not found, panic
//!     //! ifndef GREETING_STR
//!     panic!("greeting not found!");
//!     //! endif
//! }
//! ```
//! 
//! `greetings` file
//! ```
//! //! define GREETING_STR "Hello, world!"
//! ```
//! 
//! as a result, we will get code that will display the greeting specified in the "greetings" file;
//! if the drive is not found, the program will panic.
//! 
//! ### Errors
//! To describe preprocessor errors, there is a special enumeration `PreprocessorErrorType`.

use std::{collections::HashMap, fs};

use log::*;

/// Responsible for preprocessor token types.
#[derive(Debug, PartialEq, Eq)]
pub enum ShaderToken {
    /// Preprocessor command token
    DirectiveToken(usize, Directive),

    Comment(usize),

    /// Word literal (string without spaces)
    NameLit(usize, String),
    /// String literal (begins and ends with quotes, can contain any characters except quotes)
    StrLit(usize, String)
}

/// Listing preprocessor commands.
#[derive(Debug, PartialEq, Eq)]
pub enum Directive {
    /// Include directive
    /// 
    /// # Examles
    /// `main` file
    /// ```
    /// //! include "test"
    /// ``` 
    /// 
    /// `test` file
    /// ```
    /// //! ifndef _test
    /// //! define _test
    /// 
    /// fn main() {
    ///     println!("Hello, world!");
    /// }
    /// 
    /// //! endif
    /// ```
    /// 
    /// # Errors
    /// - InvalidToken
    /// - ShaderNotFound
    Include,

    /// Define directive
    /// 
    /// # Examples
    /// ```
    /// //! define VARIABLE "fn test() {}"
    /// 
    /// //! insert "VARIABLE"
    /// 
    /// fn main() {
    ///     test();
    /// }
    /// ```
    /// 
    /// # Errors
    /// - InvalidToken
    Define,

    /// UnDefine directive
    /// 
    /// # Examples
    /// ```
    /// //! define VAR ""
    /// 
    /// //! ifdef VAR
    /// // the code will be included here
    /// //! endif
    /// 
    /// //! undef VAR
    /// 
    /// //! ifndef VAR
    /// // the code here will also be included
    /// //! endif
    /// ```
    /// 
    /// # Errors
    /// - InvalidToken
    UnDefine,

    /// Insert directive
    /// 
    /// # Examples
    /// ```
    /// //! define VARIABLE "fn test() {}"
    /// 
    /// //! insert "VARIABLE"
    /// 
    /// fn main() {
    ///     test();
    /// }
    /// ```
    /// 
    /// # Errors
    /// - InvalidToken
    Insert,

    /// If define directive
    /// 
    /// # Examples
    /// ```
    /// //! define VAR ""
    /// 
    /// //! ifdef VAR
    /// // the code will be included here
    /// //! endif
    /// ```
    /// 
    /// # Errors
    /// - InvalidToken
    IfDefine,

    /// If not define directive
    /// 
    /// # Examples
    /// ``` 
    /// //! ifndef VAR
    /// // the code will be included here
    /// //! endif
    /// ```
    /// 
    /// # Errors
    /// - InvalidToken
    IfNotDefine,

    /// End if derective
    /// 
    /// # Examples
    /// ``` 
    /// //! ifndef VAR
    /// // the code will be included here
    /// //! endif
    /// ```
    /// 
    /// # Errors
    /// - IfTokenNotFound
    EndIf,
}

/// Preprocessor errors enum
#[derive(Debug)]
pub enum PreprocessorErrorType {
    /// Thrown by preprocessor commands if the argument token type does not match the desired one
    InvalidToken(usize, String, ShaderToken, ShaderToken),
    /// Thrown by the endif directive if no `ifdef` or `ifndef` was found before it
    IfTokenNotFound(usize, String),

    /// Thrown by the include directive if the shader name passed in the arguments is not a valid shader name
    IncludeShaderNotFound(usize, String, String),
    ShaderNotFound(String),
}

impl ToString for PreprocessorErrorType {
    /// Allows you to turn any element from the PreprocessorErrorType enumeration into a readable form.
    /// 
    /// # Examples
    /// ```
    /// use std::collections::HashMap;
    /// use fast_voxel_perprocessor::{ShaderPreprocessor, PreprocessorErrorType};
    /// 
    /// let mut preprocessor = ShaderPreprocessor::new(HashMap::from([
    ///     ("main".to_string(), "//! include \"test\"".to_string())
    /// ]));
    /// 
    /// let preprocessed = preprocessor.preprocess("main".to_string(), "compiler".to_string());
    /// 
    /// if let Err(error) = preprocessed {
    ///     panic!("{}", error.to_string());
    /// } else if let Ok(source) = preprocessed {
    ///     println!("shader code: {}", source);
    /// }
    /// ```
    /// 
    /// This code will panic with an error:
    /// Preprocessor error:
    ///     Shader "test" not found!
    fn to_string(&self) -> String {
        match self {
            Self::IfTokenNotFound(line, shader_name) |
            Self::IncludeShaderNotFound(line, shader_name, _) |
            Self::InvalidToken(line, shader_name, _, _) => {
                let position = format!("Error in {}:{}", shader_name, line);
                let error = match self {
                    PreprocessorErrorType::InvalidToken(_, _, exepted, recived) => format!("Invalid token: exepted {:?} recived {:?}", exepted, recived),
                    PreprocessorErrorType::IfTokenNotFound(_, _) => "If token not found!".to_string(),
                    PreprocessorErrorType::IncludeShaderNotFound(_, _, included_shader) => format!("Shader {} not found!", included_shader),
                    _ => panic!(),
                };

                format!("{}\n   {}", position, error)
            },

            Self::ShaderNotFound(name) => format!("Shader {} not found!", name)
        }
    }
}

/// Shader preprocessor structure
pub struct ShaderPreprocessor {
    defines: HashMap<String, String>,
    shader_sources: HashMap<String, String>,
    ifs: Vec<(ShaderToken, bool)>
}

impl ShaderPreprocessor {
    /// Preprocessor structure creation function.
    /// 
    /// # Examples
    /// ```
    /// use std::collections::HashMap;
    /// use fast_voxel_perprocessor::{ShaderPreprocessor, PreprocessorErrorType};
    /// 
    /// let mut preprocessor = ShaderPreprocessor::new(HashMap::from([
    ///     ("main".to_string(), "".to_string())
    /// ]));
    /// ```
    pub fn new(shader_sources: HashMap<String, String>) -> Self {
        let mut shader_sources = shader_sources;

        for (std_file_name, std_file_source) in fast_voxel_lib::SHADER_STD.into_iter() {
            shader_sources.insert(std_file_name.to_string(), std_file_source.to_string());
        }

        Self {
            defines: HashMap::new(),
            ifs: Vec::new(),
            shader_sources
        }
    }

    /// Preprocessing function.
    /// 
    /// # Examples
    /// ```
    /// use std::collections::HashMap;
    /// use fast_voxel_perprocessor::{ShaderPreprocessor, PreprocessorErrorType};
    /// 
    /// let mut preprocessor = ShaderPreprocessor::new(HashMap::from([
    ///     ("main".to_string(), "".to_string())
    /// ]));
    /// 
    /// let preprocessed = preprocessor.preprocess()
    ///     .expect("Error to preprocess files");
    /// 
    /// println!("Preprocessed source: {}", preprocessed);
    /// ```
    pub fn preprocess(&mut self) -> Result<String, PreprocessorErrorType> {
        self.preprocess_impl("main".to_string(), "__compiler_main".to_string())
    }

    fn preprocess_impl(&mut self, shader_name: String, root_name: String) -> Result<String, PreprocessorErrorType> {
        info!("Preprocessing shader {}", shader_name);

        let source = if let Some(source) = self.shader_sources.get(&shader_name) {
            source
        } else {
            return Err(PreprocessorErrorType::ShaderNotFound(shader_name));
        };

        let tokens = self.tokenize(source)?;
        let parsed = self.parse(tokens, source.to_string(), shader_name)?;

        debug!("Returning to {}", root_name);

        Ok(parsed)
    }

    #[allow(unused_assignments)]
    fn tokenize(&self, source: &String) -> Result<Vec<ShaderToken>, PreprocessorErrorType> {
        debug!("Lexing");

        let mut tokens = Vec::<ShaderToken>::new();
        let preprocessor_commands = source.lines();

        for (line_num, line_src) in preprocessor_commands.enumerate() {
            if !line_src.trim_start().starts_with("//!") {
                continue;
            }

            let line_tokens = line_src
                .trim_start()
                .trim_end()
                .split(' ');

            let mut str_lit_tmp = "".to_owned();
            let mut is_str_lit = false;

            for token in line_tokens {
                if is_str_lit {
                    str_lit_tmp += token;
                }

                if token.starts_with('"') {
                    str_lit_tmp = "".to_string();
                    is_str_lit = true;

                    str_lit_tmp += token;
                }

                if token.ends_with('"') {
                    is_str_lit = false;
                    tokens.push(ShaderToken::StrLit(line_num, str_lit_tmp.replace("\"", "")));
                    trace!("Parsed token: {:?}", ShaderToken::StrLit(line_num, str_lit_tmp.replace("\"", "")));
                    continue;
                }

                if is_str_lit {
                    str_lit_tmp += " ";
                    continue;
                }

                let parsed_token = match token {
                    "//!" => continue,

                    "include" => ShaderToken::DirectiveToken(line_num, Directive::Include),
                    "define" => ShaderToken::DirectiveToken(line_num, Directive::Define),
                    "undef" => ShaderToken::DirectiveToken(line_num, Directive::UnDefine),
                    "insert" => ShaderToken::DirectiveToken(line_num, Directive::Insert),

                    "ifdef" => ShaderToken::DirectiveToken(line_num, Directive::IfDefine),
                    "ifndef" => ShaderToken::DirectiveToken(line_num, Directive::IfNotDefine),
                    "endif" => ShaderToken::DirectiveToken(line_num, Directive::EndIf),

                    "//" => ShaderToken::Comment(line_num),

                    _ => ShaderToken::NameLit(line_num, token.to_owned())
                };

                trace!("Parsed token: {:?}", parsed_token);
                tokens.push(parsed_token);
            }
        }
        
        Ok(tokens)
    }

    fn parse(&mut self, shader_tokens: Vec<ShaderToken>, source: String, this_shader_name: String) -> Result<String, PreprocessorErrorType> {
        debug!("Parsing");

        let mut tokens = shader_tokens.into_iter().peekable();
        let mut source_lines: Vec<String> = source.lines().map(|x| x.to_string()).collect();
        let mut offset: usize = 0;
        
        while tokens.peek().is_some() {
            let directive = tokens.next().unwrap();
            trace!("Found directive: {:?}", directive);

            // TODO: write comment implementation
            // if let ShaderToken::Comment(line) = directive {
            //     trace!("Found comment: {:?}", directive);

            //     while line == {
            //         let comment_token = tokens.peek().unwrap();

            //         match comment_token {
            //             ShaderToken::Comment(line) => *line,
            //             ShaderToken::DirectiveToken(line, _) => *line,
            //             ShaderToken::NameLit(line, _) => *line,
            //             ShaderToken::StrLit(line, _) => *line
            //         }
            //     } {
            //         directive = tokens.next().unwrap();
            //         trace!("Token {:?} scipped, its comment data", directive);
            //     }
            // }
    
            if let ShaderToken::DirectiveToken(_directive_line, ref directive_type) = directive {
                let directive_line = _directive_line + offset;

                match directive_type {
                    Directive::Include => {
                        let shader = tokens.next().unwrap();
    
                        if let ShaderToken::StrLit(_, shader_name) = shader {
                            let preprocessed_shader = match self.preprocess_impl(shader_name, this_shader_name.clone()) {
                                Ok(source) => source,
                                Err(PreprocessorErrorType::ShaderNotFound(name)) => {
                                    return Err(PreprocessorErrorType::IncludeShaderNotFound(
                                        _directive_line,
                                        this_shader_name,
                                        name
                                    ));
                                },
                                Err(error) => return Err(error)
                            };
                            offset += preprocessed_shader.lines().count();

                            for (shader_num, shader_line) in preprocessed_shader.lines().enumerate() {
                                source_lines.insert(directive_line + shader_num, shader_line.to_string());
                            }
                        } else {
                            return Err(PreprocessorErrorType::InvalidToken(
                                _directive_line,
                                this_shader_name,
                                ShaderToken::StrLit(0, "".to_string()),
                                shader
                            ));
                        }
                    },
                    Directive::Define => {
                        let name = tokens.next().unwrap();
                        
                        if let ShaderToken::NameLit(_, name_parsed) = name {
                            let value_option = tokens.next();

                            if let Option::Some(value) = value_option {
                                if let ShaderToken::StrLit(_, value_parsed) = value {
                                    self.defines.insert(name_parsed.to_string(), value_parsed.to_string());
                                } else {
                                    return Err(PreprocessorErrorType::InvalidToken(
                                        _directive_line,
                                        this_shader_name,
                                        ShaderToken::StrLit(0, "".to_string()),
                                        value
                                    ));
                                }
                            } else {
                                self.defines.insert(name_parsed.to_string(), "".to_string());
                            }

                        } else {
                            return Err(PreprocessorErrorType::InvalidToken(
                                _directive_line,
                                this_shader_name,
                                ShaderToken::NameLit(0, "".to_string()),
                                name
                            ));
                        }
                    },
                    Directive::UnDefine => {
                        let name = tokens.next().unwrap();
    
                        if let ShaderToken::NameLit(_, name_parsed) = name {
                            self.defines.remove(&name_parsed);
                        } else {
                            return Err(PreprocessorErrorType::InvalidToken(
                                _directive_line,
                                this_shader_name,
                                ShaderToken::NameLit(0, "".to_string()),
                                name
                            ));
                        }
                    },
                    Directive::Insert => {
                        let code = tokens.next().unwrap();

                        if let ShaderToken::StrLit(_, code_parsed) = code {
                            let mut code_preprocessed = code_parsed;

                            for (name, value) in &self.defines {
                                code_preprocessed = code_preprocessed.replace(name, value);
                            }

                            source_lines[directive_line] = code_preprocessed;
                        } else {
                            return Err(PreprocessorErrorType::InvalidToken(
                                _directive_line,
                                this_shader_name,
                                ShaderToken::StrLit(0, "".to_string()),
                                code
                            ));
                        }
                    },
                    Directive::IfDefine | Directive::IfNotDefine => {
                        let name = tokens.next().unwrap();

                        if let ShaderToken::NameLit(_, name_parsed) = name {
                            self.ifs.push((directive, self.defines.contains_key(&name_parsed)));
                        } else {
                            return Err(PreprocessorErrorType::InvalidToken(
                                _directive_line,
                                this_shader_name,
                                ShaderToken::NameLit(0, "".to_string()),
                                name
                            ));
                        }
                    },
                    Directive::EndIf => {
                        let (if_type, is_true) = if let Some((if_type, is_true)) = self.ifs.pop() {
                            (if_type, is_true)
                        } else {
                            return Err(PreprocessorErrorType::IfTokenNotFound(
                                _directive_line,
                                this_shader_name,
                            ));
                        };
                        
                        match if_type {
                            ShaderToken::DirectiveToken(if_line, Directive::IfDefine) => {
                                if is_true {
                                    continue;
                                }

                                for i in (if_line)..=directive_line {
                                    source_lines[i] = "".to_string();
                                }
                            },
                            ShaderToken::DirectiveToken(if_line, Directive::IfNotDefine) => {
                                if !is_true {
                                    continue;
                                }

                                for i in (if_line)..=directive_line {
                                    source_lines[i] = "".to_string();
                                }
                            },
                            _ => {}
                        }
                    },
                }
            } else {
                let line = match directive {
                    ShaderToken::DirectiveToken(line, _) => line,
                    ShaderToken::Comment(line) => line,
                    ShaderToken::NameLit(line, _) => line,
                    ShaderToken::StrLit(line, _) => line,
                };

                return Err(PreprocessorErrorType::InvalidToken(
                    line,
                    this_shader_name,
                    ShaderToken::DirectiveToken(0, Directive::Include),
                    directive
                ));
            }
        }
    
        Ok(source_lines.join("\n"))
    }
}

fn read_dir(path: String) -> HashMap<String, String> {
    let mut shaders = HashMap::<String, String>::new();

    for element in fs::read_dir(path).expect("Failed to read shaders path!") {
        let element = element.unwrap();

        trace!("{:?}", element);

        let name = element.file_name().into_string().unwrap().split('.').next().unwrap().to_string();
        let mut source = "".to_string();
        
        if let Ok(_source) = fs::read_to_string(element.path()) {
            source = _source;
        }

        shaders.insert(name, source);
    }

    shaders
}

/// Allows you to process directories.
/// 
/// preprocessing will be applied to all files in the folder
/// and they will be placed in the preprocessor namespace with their real name without extension.
/// 
/// # Examples
/// ```
/// use fast_voxel_perprocessor::preprocess_dir;
/// 
/// let preprocessed = preprocess_dir("shaders").unwrap();
/// 
/// // ...
/// ```
pub fn preprocess_dir(path: String) -> Result<String, PreprocessorErrorType> {
    let shaders = read_dir(path);
    let mut shader_preprocessor = ShaderPreprocessor::new(shaders);

    shader_preprocessor.preprocess()
}

// TODO: write tests