macro_rules! struct_events {
    // The below pattern expects to literally receive as its arguments (keyboard:{...}) where the ... is a any number of comma separated
    // arguments (hence the ,* which is regex for expect the pattern to occur any number of times (*) and each time will be separated by a comma (,*))
    // The right hand side of keyboard is expected to alays be a colon separated pair i.e. key_escape: Escape and our macro will assign a variable inside
    // of itself called $k_alias which at runtime using our example would be bound to equal key_escape.  The :ident is Rust's macro syntax for letting us
    // know that the object being passed in is an identifier like x = rather than what we saw earlier which was :expr telling us that the $ bound variable inside
    // the macro is an expression like 2 + 5 or true.
    (
        // This is the primary pattern that we expect our Macro to be instantiated with discussed above
        keyboard: { $( $k_alias:ident : $k_sdl:ident ),* },

        // If it is not instantiated with that pattern use this catch all identified by the else: keyword
        // We still expect to receive any number of : seperated arguments but no keyboard: {...} structure containing them and we expect the right hand side
        // of the argument to be a pattern not an identifier or expression as previously seen, i.e. Some(t); (117, 'a');
        else: { $( $e_alias:ident : $e_sdl:pat ),* }

    )

    => {
        use sdl2::EventPump;


        pub struct ImmediateEvents {
            // For every keyboard event, we have an Option<bool>
            // Some(true)  => Was just pressed
            // Some(false) => Was just released
            // None        => Nothing happening _now_

            // This is literally creating an attribute on the ImmediateEvents structure called key_escape in our example that is a Generic of type boolean (Option<bool>) meaning
            // it responds to Some(true), Some(false), and None.
            // The ,* is telling Rust to create this Option type of whatever name for every comma separated pair that got passed in, not just key_escape
            $( pub $k_alias : Option<bool> , )*

            // This handles our Quit event that isn't driven by a key press so it will always be false unless some trigger sets it to true
            $( pub $e_alias : bool ),*
        }

        impl ImmediateEvents {
            pub fn new() -> ImmediateEvents {
                ImmediateEvents {
                    // When reinitialized, nothing has yet happened, so all are
                    // set to None
                    $( $k_alias: None , )*

                    // The default for any non key based event coming in will be set to false so that we can act on the trigger of setting it to true
                    $( $e_alias: false ),*
                }
            }
        }


        pub struct Events {
            pump: EventPump,
            pub now: ImmediateEvents,

            // true  => pressed
            // false => not pressed

            //Every pair that we pass in will exist on the Events object in its current state of false (default on initialization in Events.new()) or true (set later in our code)
            // repsonding to the variable passed in as $k_alias (key_escape)
            $( pub $k_alias: bool ),*
        }

        impl Events {
            pub fn new(pump: EventPump) -> Events {
                Events {
                    pump: pump,
                    now: ImmediateEvents::new(),

                    // By default, initialize every key with _not pressed_
                    $( $k_alias: false ),*
                }
            }

            pub fn pump(&mut self) {
                self.now = ImmediateEvents::new();

                for event in self.pump.poll_iter() {
                    use sdl2::event::Event::*;
                    use sdl2::keyboard::Keycode::*;

                    match event {
                        KeyDown { keycode, .. } => match keycode {
                            // $( ... ),* containing $k_sdl and $k_alias means:
                            //   "for every element ($k_alias : $k_sdl) pair,
                            //    check whether the keycode is Some($k_sdl). If
                            //    it is, then set the $k_alias fields to true."
                            $(
                                // In our example $k_sdl is the Escape key so when the KeyDown event is triggered and passed in with a keycode of "A" nothing will happen
                                // because we only created an Escape key for our keyboard: {...} so far.  However if we had keyboard {key_escape:Escape, key_a:A} then the
                                // ,* would iterate to the key_a:A eventually setting $k_sdl to A and the "A" keypress would trigger.
                                Some($k_sdl) => {
                                    // Prevent multiple presses when keeping a key down
                                    // Was previously not pressed?
                                    if !self.$k_alias {
                                        // Key pressed
                                        self.now.$k_alias = Some(true);
                                    }

                                    self.$k_alias = true;
                                }
                            ),* // and add a comma after every option
                            _ => {}
                        },

                        KeyUp { keycode, .. } => match keycode {
                            $(
                                Some($k_sdl) => {
                                    // Key released
                                    self.now.$k_alias = Some(false);
                                    self.$k_alias = false;
                                }
                            ),*
                            _ => {}
                        },
                        // $e_sdl is set to a pattern which is also an event Quit { .. } where { .. } just means we don't care about any arguments we just care about the
                        // fact that Quit has been called.  Just like we matched the event KeyDown { keycode, ..} what we are doing here is matching the event Quit { .. }
                        $(
                            $e_sdl => {
                                // Self refers to the Event object which responds to .now by creating an ImmediateEvents object which we defined $e_alias on as an attribute
                                // which is set to quit: in main.rs when we pass in quit: Quit { .. }.  We change its default value from false to true allowing us to now watch
                                // for events.now.quit in an if statement and break out of our program immediatly after it is set to true
                                self.now.$e_alias = true;
                            }
                        )*,

                        _ => {}
                    }
                }
            }
        }
    }
}
