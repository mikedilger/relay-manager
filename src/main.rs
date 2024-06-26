mod auth;
mod error;
mod web;
mod signer;

use error::Error;
use http::uri::Uri;
use serde_json::{json, Value};
use std::env;
use std::str::FromStr;

fn post(uri: Uri, method: &str, params: Value) -> Result<(), Error> {
    let cmd = json!({
        "method": method,
        "params": params
    });
    let cmdstr = serde_json::to_string(&cmd)?;
    let value = web::post(&uri, cmdstr)?;
    println!("{}", value);
    Ok(())
}

fn main() -> Result<(), Error> {
    let mut args = env::args();
    let _ = args.next().expect("Command was not given arg[0]");
    let url_text = args
        .next()
        .expect("Usage: relay-tester <url> <command> [args] ...");
    let uri = Uri::from_str(&url_text)?;
    let command = args
        .next()
        .expect("Usage: relay-tester <url> <command> [args] ...");
    match &*command {
        "supportedmethods" | "listbannedpubkeys" | "listallowedpubkeys" | "listeventsneedingmoderation" |
        "listbannedevents" | "listallowedkinds" | "listblockedips" => post(uri, &*command, json!([]))?,
        "banpubkey" => {
            let pubkeytext = args.next().expect("Usage: relay-tester banpubkey <pubkey>");
            post(uri, &*command, json!([pubkeytext]))?
        },
        "allowpubkey" => {
            let pubkeytext = args.next().expect("Usage: relay-tester allowpubkey <pubkey>");
            post(uri, &*command, json!([pubkeytext]))?
        },
        "allowevent" => {
            let idtext = args.next().expect("Usage: relay-tester allowevent <id>");
            post(uri, &*command, json!([idtext]))?
        },
        "banevent" => {
            let idtext = args.next().expect("Usage: relay-tester banevent <id>");
            post(uri, &*command, json!([idtext]))?
        },
        "changerelayname" => {
            let newname = args.next().expect("Usage: relay-tester changerelayname <newname>");
            post(uri, &*command, json!([newname]))?
        },
        "changerelaydescription" => {
            let newdesc = args.next().expect("Usage: relay-tester changerelaydescription <newname>");
            post(uri, &*command, json!([newdesc]))?
        },
        "changerelayicon" => panic!("changerelayicon is unimplemented"),
        "allowkind" => {
            let kind = args.next().expect("Usage: relay-tester allowkind <kind>");
            post(uri, &*command, json!([kind]))?
        },
        "disallowkind" => {
            let kind = args.next().expect("Usage: relay-tester disallowkind <kind>");
            post(uri, &*command, json!([kind]))?
        },
        "blockip" => {
            let ip = args.next().expect("Usage: relay-tester blockip <ip>");
            post(uri, &*command, json!([ip]))?
        },
        "unblockip" => {
            let ip = args.next().expect("Usage: relay-tester unblockip <ip>");
            post(uri, &*command, json!([ip]))?
        },
        _ => panic!("Usage relay-tester <command> [args]...\n
Available Commands:  supportedmethods, banpubkey, listbannedpubkeys, allowpubkey, listallowedpubkeys, listeventsneedingmoderation, allowevent, banevent, listbannedevents, changerelayname, changerelaydescription, changerelayicon, allowkind, disallowkind, listallowedkinds, blockip, unblockip, listblockedips"),
    }

    Ok(())
}
