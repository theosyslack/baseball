pub struct Field {
    bases: [FieldBase; 3]
}

struct FieldBase {
    status: Option<Player>,
    fielder: Player
}