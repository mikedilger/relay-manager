mod auth;
mod error;
mod web;
mod signer;

use error::Error;
use http::uri::Uri;
use serde_json::json;
use std::env;
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let mut args = env::args();
    let _ = args.next().expect("Command was not given arg[0]");
    let url_text = args
        .next()
        .expect("Usage: relay-tester <url> <command> [args] ...");
    let url = Uri::from_str(&url_text)?;
    let command = args
        .next()
        .expect("Usage: relay-tester <url> <command> [args] ...");
    match &*command {
        "supportedmethods" => supportedmethods(url)?,
        "banpubkey" => {
            let pubkeytext = args.next().expect("Usage: relay-tester banpubkey <pubkey>");
            banpubkey(url, pubkeytext)?;
        },
        "listbannedpubkeys" => listbannedpubkeys(url)?,
        "allowpubkey" => {
            let pubkeytext = args.next().expect("Usage: relay-tester allowpubkey <pubkey>");
            allowpubkey(url, pubkeytext)?;
        },
        "listallowedpubkeys" => listallowedpubkeys(url)?,
        "listeventsneedingmoderation" => listeventsneedingmoderation(url)?,
        "allowevent" => {
            let idtext = args.next().expect("Usage: relay-tester allowevent <id>");
            allowevent(url, idtext)?;
        },
        "banevent" => {
            let idtext = args.next().expect("Usage: relay-tester banevent <id>");
            banevent(url, idtext)?;
        },
        "listbannedevents" => listbannedevents(url)?,
        "changerelayname" => {
            let newname = args.next().expect("Usage: relay-tester changerelayname <newname>");
            changerelayname(url, newname)?;
        },
        "changerelaydescription" => {
            let newname = args.next().expect("Usage: relay-tester changerelaydescription <newname>");
            changerelaydescription(url, newname)?;
        },
        "changerelayicon" => panic!("changerelayicon is unimplemented"),
        "allowkind" => {
            let kind = args.next().expect("Usage: relay-tester allowkind <kind>");
            allowkind(url, kind)?;
        },
        "disallowkind" => {
            let kind = args.next().expect("Usage: relay-tester disallowkind <kind>");
            disallowkind(url, kind)?;
        },
        "listallowedkinds" => listallowedkinds(url)?,
        "blockip" => {
            let ip = args.next().expect("Usage: relay-tester blockip <ip>");
            blockip(url, ip)?;
        },
        "unblockip" => {
            let ip = args.next().expect("Usage: relay-tester unblockip <ip>");
            unblockip(url, ip)?;
        },
        "listblockedips" => listblockedips(url)?,
        _ => panic!("Usage relay-tester <command> [args]...\n
Available Commands:  supportedmethods, banpubkey, listbannedpubkeys, allowpubkey, listallowedpubkeys, listeventsneedingmoderation, allowevent, banevent, listbannedevents, changerelayname, changerelaydescription, changerelayicon, allowkind, disallowkind, listallowedkinds, blockip, unblockip, listblockedips"),
    }

    Ok(())
}

fn supportedmethods(url: Uri) -> Result<(), Error> {
    let cmd = json!({
        "method": "supportedmethods",
        "params": []
    });
    let cmdstr = serde_json::to_string(&cmd)?;
    let value = web::post(&url, cmdstr)?;
    println!("{}", value);
    Ok(())
}

fn banpubkey(_url: Uri, _pubkeytext: String) -> Result<(), Error> {
    unimplemented!()
}

fn listbannedpubkeys(_url: Uri) -> Result<(), Error> {
    unimplemented!()
}

fn allowpubkey(_url: Uri, _pubkeytext: String) -> Result<(), Error> {
    unimplemented!()
}

fn listallowedpubkeys(_url: Uri) -> Result<(), Error> {
    unimplemented!()
}

fn listeventsneedingmoderation(_url: Uri) -> Result<(), Error> {
    unimplemented!()
}

fn allowevent(_url: Uri, _idtext: String) -> Result<(), Error> {
    unimplemented!()
}

fn banevent(_url: Uri, _idtext: String) -> Result<(), Error> {
    unimplemented!()
}

fn listbannedevents(_url: Uri) -> Result<(), Error> {
    unimplemented!()
}

fn changerelayname(_url: Uri, _newname: String) -> Result<(), Error> {
    unimplemented!()
}

fn changerelaydescription(_url: Uri, _newdesc: String) -> Result<(), Error> {
    unimplemented!()
}

fn allowkind(_url: Uri, _kindtext: String) -> Result<(), Error> {
    unimplemented!()
}

fn disallowkind(_url: Uri, _kindtext: String) -> Result<(), Error> {
    unimplemented!()
}

fn listallowedkinds(_url: Uri) -> Result<(), Error> {
    unimplemented!()
}

fn blockip(_url: Uri, _iptext: String) -> Result<(), Error> {
    unimplemented!()
}

fn unblockip(_url: Uri, _iptext: String) -> Result<(), Error> {
    unimplemented!()
}

fn listblockedips(_url: Uri) -> Result<(), Error> {
    unimplemented!()
}
