use bevy::prelude::*;
use bevy_egui::egui::Margin;
use belly::prelude::*;
pub struct GameUI;

impl Plugin for GameUI{
    fn build(&self, app: &mut App){
        app.add_systems(Startup,spawn_game_ui);
    }
}

fn spawn_game_ui(mut commands: Commands){
    commmands.add(eml!{
        <body s:padding="50px">
            "Hello world"
            </body>
    });
        
    
}