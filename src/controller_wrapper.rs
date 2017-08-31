use sdl2::Sdl;
use sdl2::controller::GameController;

pub struct ControllerWrapper {
    controllers:Vec<GameController>
}

impl ControllerWrapper {

    pub fn new (sdl_context: &Sdl) -> ControllerWrapper {

        let gc_subsystem = sdl_context.game_controller().unwrap();
        let available = gc_subsystem.num_joysticks().unwrap();
        let mut controllers_found: Vec<GameController> = Vec::new();

        // Iterate over all available joysticks and look for game controllers.
        for id in 0..available {
            if gc_subsystem.is_game_controller(id) {
                match gc_subsystem.open(id) {
                    Ok(c) => {
                        println!("Success: opened \"{}\"", c.name());
                        controllers_found.push(Some(c).unwrap());
                    },
                    Err(e) => println!("failed: {:?}", e),
                }
            } else {
                 println!("{} is not a game controller", id);
            }
        }

        ControllerWrapper { controllers: controllers_found }
    }

    pub fn print_mapping (&self) {
        for controller in &self.controllers {
            println!("Controller mapping: {}", controller.mapping());
        }
    }

    pub fn number_of_controllers (&self) -> usize {
        self.controllers.len()
    }

    pub fn get_controller (&self, controller_number:usize) -> Result<&GameController, &str> {
        if self.controllers.is_empty() {
            return Err("There are currently no game controllers availeble!");
        }
        if controller_number > self.number_of_controllers() {
            Err("There are less controllers availeble a the controller number requested!") }
        else {
            Ok (&self.controllers[controller_number])
        }
    }

    /*
    for event in ctx.event_pump().unwrap().wait_iter() {


        match event {
            Event::ControllerAxisMotion{ axis, value: val, .. } => {
                // Axis motion is an absolute value in the range
                // [-32768, 32767]. Let's simulate a very rough dead
                // zone to ignore spurious events.
                let dead_zone = 10000;
                if val > dead_zone || val < -dead_zone {
                    println!("Axis {:?} moved to {}", axis, val);
                }
            }
            Event::ControllerButtonDown{ button, .. } =>
                println!("Button {:?} down", button),
            Event::ControllerButtonUp{ button, .. } =>
                println!("Button {:?} up", button),
            Event::Quit{..} => break,
            _ => (),
        }
    }*/
}
