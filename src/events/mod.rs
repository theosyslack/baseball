mod kind;

use self::kind::EventKind;

pub struct Event {
    kind: EventKind
}

impl Event {
    fn new (kind: EventKind) -> Self {
        Event {
            kind
        }
    }
}



