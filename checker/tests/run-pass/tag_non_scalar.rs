// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// A test for adding tags to non-scalar values and checking tags on sub-components

#![feature(const_generics)]
#![allow(incomplete_features)]

#[macro_use]
extern crate mirai_annotations;

use mirai_annotations::{TagPropagation, TagPropagationSet};

struct SecretTaintKind<const MASK: TagPropagationSet> {}

const SECRET_TAINT: TagPropagationSet = tag_propagation_set!(TagPropagation::SubComponent);

type SecretTaint = SecretTaintKind<SECRET_TAINT>;

pub mod intra_procedure {
    use crate::SecretTaint;

    pub fn test_tuple() {
        let tuple = (1, 2, 3, 4);
        add_tag!(&tuple, SecretTaint);
        verify!(
            has_tag!(&tuple.0, SecretTaint)
                && has_tag!(&tuple.1, SecretTaint)
                && has_tag!(&tuple.2, SecretTaint)
                && has_tag!(&tuple.3, SecretTaint)
        );
    }

    pub fn test_array() {
        let array = [1, 2, 3, 4];
        add_tag!(&array, SecretTaint);
        verify!(
            has_tag!(&array[0], SecretTaint)
                && has_tag!(&array[1], SecretTaint)
                && has_tag!(&array[2], SecretTaint)
                && has_tag!(&array[3], SecretTaint)
        );
    }

    pub fn test_slice() {
        let array = [1, 2, 3, 4];
        let slice: &[i32] = &array;
        add_tag!(slice, SecretTaint);
        verify!(
            has_tag!(&slice[0], SecretTaint)
                && has_tag!(&slice[1], SecretTaint)
                && has_tag!(&slice[2], SecretTaint)
                && has_tag!(&slice[3], SecretTaint)
        );
    }
}

pub mod inter_procedure {
    use crate::SecretTaint;

    pub fn test_tuple(tuple: &(i32, i32, i32, i32)) {
        precondition!(has_tag!(tuple, SecretTaint));
        verify!(
            has_tag!(&tuple.0, SecretTaint)
                && has_tag!(&tuple.1, SecretTaint)
                && has_tag!(&tuple.2, SecretTaint)
                && has_tag!(&tuple.3, SecretTaint)
        );
    }

    pub fn test_array(array: &[i32; 4]) {
        precondition!(has_tag!(array, SecretTaint));
        verify!(
            has_tag!(&array[0], SecretTaint)
                && has_tag!(&array[1], SecretTaint)
                && has_tag!(&array[2], SecretTaint)
                && has_tag!(&array[3], SecretTaint)
        );
    }

    pub fn test_slice(slice: &[i32]) {
        precondition!(slice.len() == 4);
        precondition!(has_tag!(slice, SecretTaint));
        verify!(
            has_tag!(&slice[0], SecretTaint)
                && has_tag!(&slice[1], SecretTaint)
                && has_tag!(&slice[2], SecretTaint)
                && has_tag!(&slice[3], SecretTaint)
        );
    }
}
