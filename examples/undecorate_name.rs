// Copyright 2024 Ryan McKenzie
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::Parser;
use undname::Flags;

#[derive(Parser)]
struct Cli {
    mangled_string: String,

    #[arg(long)]
    no_calling_convention: bool,

    #[arg(long)]
    no_tag_specifier: bool,

    #[arg(long)]
    no_access_specifier: bool,

    #[arg(long)]
    no_member_type: bool,

    #[arg(long)]
    no_return_type: bool,

    #[arg(long)]
    no_variable_type: bool,

    #[arg(long)]
    no_this_type: bool,

    #[arg(long)]
    no_leading_underscores: bool,

    #[arg(long)]
    no_ms_keywords: bool,

    #[arg(long)]
    name_only: bool,
}

fn main() {
    let cli = Cli::parse();
    let flags = {
        let mut flags = Flags::empty();
        if cli.no_calling_convention {
            flags |= Flags::NO_CALLING_CONVENTION;
        }
        if cli.no_tag_specifier {
            flags |= Flags::NO_TAG_SPECIFIER;
        }
        if cli.no_access_specifier {
            flags |= Flags::NO_ACCESS_SPECIFIER;
        }
        if cli.no_member_type {
            flags |= Flags::NO_MEMBER_TYPE;
        }
        if cli.no_return_type {
            flags |= Flags::NO_RETURN_TYPE;
        }
        if cli.no_variable_type {
            flags |= Flags::NO_VARIABLE_TYPE;
        }
        if cli.no_this_type {
            flags |= Flags::NO_THISTYPE;
        }
        if cli.no_leading_underscores {
            flags |= Flags::NO_LEADING_UNDERSCORES;
        }
        if cli.no_ms_keywords {
            flags |= Flags::NO_MS_KEYWORDS;
        }
        if cli.name_only {
            flags |= Flags::NAME_ONLY;
        }
        flags
    };

    let mangled_string = cli.mangled_string;
    println!("{mangled_string}");
    let result = undname::demangle(&mangled_string, flags);
    match result {
        Ok(ok) => println!("{ok}"),
        Err(_) => println!("error: Invalid mangled name"),
    }
}
