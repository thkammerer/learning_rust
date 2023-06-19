extern crate multimap;
use multimap::MultiMap;

pub struct Event {
    event_type : String,
    event_msg: String
}

pub trait ISubscriber {
    fn handle_event( &self, event: &Event );
}

pub trait IPublisher {
    fn publish( event: Event );
}

trait IEventChannel<'a, T: ISubscriber> {
    fn attach(&mut self, event_type: String , subscriber: &'a T);
    fn detach_all(&mut self);
    fn publish(&self, event: Event );
    fn len(&self) ->usize;
}

pub struct EventChannel<'a, T: ISubscriber> {
    name: String,
    event_handlers: MultiMap<String, &'a T>
}

impl <'a, T: ISubscriber + PartialEq> EventChannel<'a, T> {
    fn new( name: String) -> Self {
        Self {
            name,
            event_handlers: MultiMap::new()
        }
    }
}

impl <'a, T: ISubscriber + PartialEq> IEventChannel<'a, T> for EventChannel<'a, T> {
    
    fn attach(&mut self, event_type: String, subscriber: &'a T) {
        self.event_handlers.insert(event_type, subscriber);
    }

    fn detach_all(&mut self) {
            self.event_handlers.clear();
    }

    fn publish(&self, event: Event) {
        if self.event_handlers.contains_key(&event.event_type) {
            let vec = self.event_handlers.get_vec(&event.event_type).unwrap();
            for subscriber in vec {
               subscriber.handle_event(&event);
            }
        }        
    }

    fn len(&self) -> usize {
        self.event_handlers.len()
   }
}

pub struct PrinterSubscriber {
    name: String,
}

impl PrinterSubscriber {
    fn new ( name: String ) -> Self {
        Self {
            name,
        }
    }
}

impl ISubscriber for PrinterSubscriber {
    fn handle_event( &self, event: &Event ) {       
        println!(" PrinterSubscriber: {} received event type: {} and event{}", self.name, event.event_type, event.event_msg);
    }
}


pub struct Sensor {
    name: String,
    value: i32,
}

impl Sensor  {        
    fn new( name: String ) -> Self {
        Self {        
            name,
            value : 0,
        }
    }
    fn set_value( &mut self, new_value: i32) {
        self.value = new_value;
    }
}

impl IPublisher for Sensor {
    fn publish( event: Event ) {
        todo!()
    }
}


// Tests --------------------------------------------------------------------------
mod test {
    use super::*;

    #[test]
    fn create_subject_two_observers_attach_and_do_something() {
//        let event_channel = EventChannel::new( String::from("test event channel"));

    }
}

