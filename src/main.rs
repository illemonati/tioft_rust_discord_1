extern crate serenity;

use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use std::env;
use serenity::utils::MessageBuilder;
use serenity::model::channel::Embed;
use serenity::utils::Colour;
use serenity::model::channel::EmbedImage;

const PREFIX : &str =  "@@";
struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {

    let RED:Colour = Colour::from_rgb(204,0,0);


       if (&msg).content.trim().to_lowercase().contains("alex"){
           match msg.channel_id.say("He be tard. --Jon Skeet") {
                Ok(_) => {},
                Err(e) => {println!("Error: {}", e)},
           }
       }

       if is_command(&msg.content, "ping"){
           match msg.channel_id.say("Pong"){
               Ok(_) => {},
               Err(e) => {println!("Error: {}", e)},
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
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");
    if let Err(why) = client.start_shards(1) {
        println!("Client error: {:?}", why);
    }
}


fn is_command(message:&String, command_name:&str) -> bool{
    message.trim() == String::from(PREFIX) + command_name
}
