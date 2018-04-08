/**
 * fhistory - https://github.com/asmuth/fhistory
 * Copyright (c) 2018, Paul Asmuth <paul@asmuth.com>
 *
 * This file is part of the "fhistory" project. fhistory is free software
 * licensed under the Apache License, Version 2.0 (the "License"); you may not
 * use this file except in compliance with the License.
 */
#[derive(Debug)]
pub enum Operation {
  Acknowledge,
  Status,
  Verify,
  History,
  Initialize
}

pub trait OperationHelp {
  fn usage(self: &Self) -> String;
}

impl Operation {

  pub fn from_str(str: &str) -> Option<Operation> {
    return match str {
      "ack" => Some(Operation::Acknowledge),
      "status" => Some(Operation::Status),
      "verify" => Some(Operation::Verify),
      "init" => Some(Operation::Initialize),
      "log" => Some(Operation::History),
      _ => None,
    };
  }

}
