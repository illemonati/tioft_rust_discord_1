extern crate serenity;

use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use std::env;
use serenity::utils::MessageBuilder;
use serenity::model::channel::Embed;
use serenity::utils::Colour;
use serenity::model::channel::EmbedImage;
use std::fs::File;
use std::time::Instant;


const PREFIX : &str =  "@@";
struct Handler;



impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {

        let RED:Colour = Colour::from_rgb(204,0,0);

           // println!("{:?}\n\n\n",&msg );

        if (&msg).content.trim().to_lowercase().contains("alex"){
           match msg.channel_id.say("He be tard. --Jon Skeet") {
                Ok(_) => {},
                Err(e) => {eprintln!("Error: {}", e);},
           }
        }

        if is_command(&msg.content, "say"){
           let message : Vec<&str>= msg.content.trim().split_whitespace().collect();
           let message : String = String::from(message[1..].join(" "));
           match msg.channel_id.say(message){
               Ok(_) => {},
               Err(e) => {eprintln!("Error: {}", e)},
           }
        }

        if is_command(&msg.content, "ping"){
           match msg.channel_id.say("Pong"){
               Ok(_) => {},
               Err(e) => {eprintln!("Error: {}", e)},
           }
        }

        if is_command(&msg.content, "communism"){
           let communism_text = "A spectre is haunting Europe — the spectre of communism. All the powers of old Europe have entered into a holy alliance to exorcise this spectre: Pope and Tsar, Metternich and Guizot, French Radicals and German police-spies.Where is the party in opposition that has not been decried as communistic by its opponents in power? Where is the opposition that has not hurled back the branding reproach of communism, against the more advanced opposition parties, as well as against its reactionary adversaries?Two things result from this fact:I. Communism is already acknowledged by all European powers to be itself a power.II. It is high time that Communists should openly, in the face of the whole world, publish their views, their aims, their tendencies, and meet this nursery tale of the Spectre of Communism with a manifesto of the party itself.To this end, Communists of various nationalities have assembled in London and sketched the following manifesto, to be published in the English, French, German, Italian, Flemish and Danish languages";
           let communism_url = "https://www.marxists.org/archive/marx/works/1848/communist-manifesto/ch01.htm";
           let img = "http://www.fm-base.co.uk/forum/attachments/transfer-updates-custom-leagues-editing/211761d1324568367-ussr-yugoslavia-leagues-national-sides-database-soviet_union_ussr_grunge_flag_by_think0.jpg";
           match msg.channel_id.send_message(|m|m.embed(|e| e
               .title("Manifesto of the Communist Party")
               .description(&communism_text)
               .color(RED)
               .image(&img)
               .url(&communism_url))){
                Ok(_) => {},
                Err(e) => {println!("Error: {}", e)},
            }
        }

            fn print_msg(msg:& Message) {
                println!("-------------------------------------------------------------------------");
                println!("id: {}\n",(&msg).id);
                println!("content: {}\n",(&msg).content);
                println!("content_debug: {:?}\n",(&msg).content);
                println!("embeds: {:?}\n",(&msg).embeds);
                println!("attachments: {:?}\n",(&msg).attachments);
                println!("author: {}\n",(&msg).author);
                println!("author_debug: {:?}\n",(&msg).author);
                // println!("{:?}",(&msg).bot);
                // println!("{:?}",(&msg).name);
                // println!("{:?}",(&msg).discriminator);
                println!("channel_id: {}\n",(&msg).channel_id);
                println!("guild_id: {:?}\n",(&msg).guild_id);
                println!("kind: {:?}\n",(&msg).kind);
                // println!("{:?}",(&msg).memeber);
                println!("mention_everyone: {}\n",(&msg).mention_everyone);
                println!("mention_roles: {:?}\n",(&msg).mention_roles);
                println!("mentions: {:?}\n",(&msg).mentions);
                println!("tts: {}\n",(&msg).tts);
                println!("webhook_id: {:?}\n",(&msg).webhook_id);
                println!("timestamp: {}\n",(&msg).timestamp);
                println!("-------------------------------------------------------------------------");
                print!("\n\n\n");
            }

            print_msg(&msg);
        }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!\n", ready.user.name);
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");
    if let Err(why) = client.start_shards(1) {
        println!("Client error: {:?}", why);
    }
    let now = Instant::now();
}


fn is_command(message:&String, command_name:&str) -> bool{
    // message.trim() == String::from(PREFIX) + command_name
    let message : Vec<&str>= message.trim().split_whitespace().collect();
    // match ([&message[0], message[1]].join(" ") == String::from(PREFIX) + command_name){
    (message[0] == String::from(PREFIX) + command_name)
}
