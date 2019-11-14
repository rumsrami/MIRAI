// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use log_derive::logfn_inputs;
use mirai_annotations::assume;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use syntax::errors::Diagnostic;
use syntax_pos::MultiSpan;

/// A collection of error strings that are expected for a test case.
#[derive(Debug)]
pub struct ExpectedErrors {
    messages: Vec<String>,
}

impl ExpectedErrors {
    /// Reads the file at the given path and scans it for instances of "//~ message".
    /// Each message becomes an element of ExpectedErrors.messages.
    #[logfn_inputs(TRACE)]
    pub fn new(path: &str) -> ExpectedErrors {
        let exp = load_errors(&PathBuf::from_str(&path).unwrap());
        ExpectedErrors { messages: exp }
    }

    /// Checks if the given set of diagnostics matches the expected diagnostics.
    #[logfn_inputs(TRACE)]
    pub fn check_messages(&mut self, diagnostics: Vec<Diagnostic>) -> bool {
        for diag in diagnostics.iter() {
            if !self.remove_message(&diag.span, &diag.message()) {
                return false;
            }
            for child in &diag.children {
                if !self.remove_message(&child.span, &child.message()) {
                    return false;
                }
            }
        }
        if !self.messages.is_empty() {
            println!("Expected errors not reported: {:?}", self.messages);
            return false;
        }
        true
    }

    /// Removes the first element of self.messages and checks if it matches msg.
    #[logfn_inputs(TRACE)]
    fn remove_message(&mut self, span: &MultiSpan, msg: &str) -> bool {
        let mut cand: Option<String> = None;
        for expected in self.messages.to_owned() {
            if msg.contains(expected.as_str()) {
                // Take care of finding the longest match
                if cand.is_none() || cand.as_ref().unwrap().len() < expected.len() {
                    cand = Some(expected);
                }
            }
        }
        if let Some(expected) = cand {
            self.messages.remove_item(&expected);
            true
        } else {
            println!(
                "Unexpected error: \"{}\". Expected: {:?} (at {:?})",
                msg, self.messages, span,
            );
            false
        }
    }
}

/// Scans the contents of test file for patterns of the form "//~ message"
/// and returns a vector of the matching messages.
#[logfn_inputs(TRACE)]
fn load_errors(testfile: &Path) -> Vec<String> {
    let rdr = BufReader::new(File::open(testfile).unwrap());
    let tag = "//~";
    rdr.lines()
        .enumerate()
        .filter_map(|(_line_num, line)| parse_expected(&line.unwrap(), &tag))
        .collect()
}

/// Returns the message part of the pattern "//~ message" if there is a match, otherwise None.
#[logfn_inputs(TRACE)]
fn parse_expected(line: &str, tag: &str) -> Option<String> {
    let tag_start = line.find(tag)?;
    // If the tag has been found this following must be true.
    assume!(tag_start < usize::max_value() - tag.len());
    let start = tag_start + tag.len();
    Some(String::from(line[start..].trim()))
}
