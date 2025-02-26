/*
 * This file is part of espanso.
 *
 * Copyright (C) 2019-2021 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::process::Command;

use anyhow::Result;
use thiserror::Error;

use crate::cli::util::CommandExt;
use crate::cli::PathsOverrides;

pub fn launch_daemon(paths_overrides: &PathsOverrides) -> Result<()> {
  let espanso_exe_path = std::env::current_exe()?;
  let mut command = Command::new(&espanso_exe_path.to_string_lossy().to_string());
  command.args(&["daemon"]);
  command.with_paths_overrides(paths_overrides);

  let mut child = command.spawn()?;
  let result = child.wait()?;

  if result.success() {
    Ok(())
  } else {
    Err(DaemonError::NonZeroExitCode.into())
  }
}

#[derive(Error, Debug)]
pub enum DaemonError {
  #[error("unexpected error, 'espanso daemon' returned a non-zero exit code.")]
  NonZeroExitCode,
}
