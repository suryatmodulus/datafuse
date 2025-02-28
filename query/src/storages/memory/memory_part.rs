//  Copyright 2022 Datafuse Labs.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

use std::any::Any;
use std::sync::Arc;

use common_exception::ErrorCode;
use common_exception::Result;
use common_planners::PartInfo;
use common_planners::PartInfoPtr;

#[derive(serde::Serialize, serde::Deserialize, PartialEq)]
pub struct MemoryPartInfo {
    pub total: usize,
    pub part_start: usize,
    pub part_end: usize,
}

#[typetag::serde(name = "memory")]
impl PartInfo for MemoryPartInfo {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, info: &Box<dyn PartInfo>) -> bool {
        match info.as_any().downcast_ref::<MemoryPartInfo>() {
            None => false,
            Some(other) => self == other,
        }
    }
}

impl MemoryPartInfo {
    pub fn create(start: usize, end: usize, total: usize) -> Arc<Box<dyn PartInfo>> {
        Arc::new(Box::new(MemoryPartInfo {
            total,
            part_start: start,
            part_end: end,
        }))
    }

    pub fn from_part(info: &PartInfoPtr) -> Result<&MemoryPartInfo> {
        match info.as_any().downcast_ref::<MemoryPartInfo>() {
            Some(part_ref) => Ok(part_ref),
            None => Err(ErrorCode::LogicalError(
                "Cannot downcast from PartInfo to MemoryPartInfo.",
            )),
        }
    }
}
