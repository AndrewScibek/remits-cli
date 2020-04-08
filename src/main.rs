use clap::clap_app;
mod client;
use serde::{Deserialize, Serialize};

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.1")
        (author: "Andrew Scibek")
        (about: "Interact with remits")
        (@arg debug: -d ... "Sets the level of debugging information")
        (@subcommand log_add =>
            (about: "Add log")
            (version: "0.1")
            (author: "Andrew Scibek")
            (@arg log_name: -n +required +takes_value "Log name to add")
        )
        (@subcommand log_list =>
            (about: "List logs")
            (version: "0.1")
            (author: "Andrew Scibek")
        )
        (@subcommand log_del =>
            (about: "Delete log")
            (version: "0.1")
            (author: "Andrew Scibek")
            (@arg log_name: -n +required +takes_value "Log name to delete")
        )
        (@subcommand log_show =>
            (about: "Show metadata of log")
            (version: "0.1")
            (author: "Andrew Scibek")
            (@arg log_name: -n +required +takes_value "Log name to see metadata")
        )
        (@subcommand msg_add =>
            (about: "Add message to log")
            (version: "0.1")
            (author: "Andrew Scibek")
            (@arg msg: -m +takes_value "Value of msg to add")
        )
        (@subcommand itr_add =>
            (about: "Add itr to log")
            (version: "0.1")
            (author: "Andrew Scibek")
            (@arg log: -l +required +takes_value "Value of log to add itr")
            (@arg itr_name: -n +required +takes_value "choose itr name")
            (@arg itr_type: -t +required +takes_value "select itr type")
        )
        (@subcommand itr_list =>
            (about: "List all itrs")
            (version: "0.1")
            (author: "Andrew Scibek")
        )
        (@subcommand itr_next =>
            (about: "Get up to <count> messages from an Iterator")
            (version: "0.1")
            (author: "Andrew Scibek")
            (@arg itr_name: -n +required +takes_value "itr name")
            (@arg message_id: -i +required +takes_value "message_id")
            (@arg count: -c +required +takes_value "count")
        )
    )
    .get_matches();

    let request = match matches.subcommand() {
        ("log_list", Some(_)) => format!("{:?}", client::new_log_list_req()),
        ("log_add", Some(args)) => format!(
            "{:?} Added log: {}",
            client::new_log_add_req(args.value_of("log_name").unwrap()),
            args.value_of("log_name").unwrap()
        ),
        ("log_show", Some(args)) => format!(
            "{:?}",
            client::new_log_show_req(args.value_of("log_name").unwrap())
        ),
        ("log_del", Some(args)) => format!(
            "{:?} Deleted log: {}",
            client::new_log_del_req(args.value_of("log_name").unwrap()),
            args.value_of("log_name").unwrap()
        ),
        ("msg_add", Some(args)) => {
            #[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
            struct Msg {
                data: String,
            }
            let test_msg = Msg {
                data: args.value_of("msg").unwrap().into(),
            };
            let cbor = serde_cbor::to_vec(&test_msg).unwrap();

            format!("{:?}", client::new_msg_add_req("test", cbor))
        }
        ("itr_add", Some(args)) => {
            let log = args.value_of("log").unwrap().into();
            let itr_name = args.value_of("itr_name").unwrap().into();
            let itr_type = args.value_of("itr_type").unwrap().into();
            format!("{:?}", client::new_itr_add_req(log, itr_name, itr_type))
        }
        ("itr_list", Some(_)) => format!("{:?}", client::new_itr_list_req()),
        ("itr_next", Some(args)) => {
            let itr_name = args.value_of("itr_name").unwrap().into();
            let message_id = args.value_of("message_id").unwrap().parse().unwrap();
            let count = args.value_of("count").unwrap().parse().unwrap();
            format!(
                "{:?}",
                client::new_itr_next_req(itr_name, message_id, count)
            )
        }
        _ => panic!("{}", "Unknown commmand"),
    };
    println!("{}", request);
    // let res = send(request)
    // first byte is type 0x01:info 0x02:data 0x03:error and return raw bytes
    // Continued program logic goes here...
}
