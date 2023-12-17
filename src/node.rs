use eframe::emath::Pos2;

enum ConnectionType {
    Integer,
    IntegerArray,
    Real,
    RealArray,
    String,
    StringArray,
    ByteArray,
    Bitmap,
}

trait Port {
    fn get_id(&self) -> i8;
    fn get_type(&self) -> ConnectionType;
}

struct InPort {

}

struct OutPort {

}

trait Visual {
    fn get_bounds(&self) -> Pos2;
    fn get_pos(&self) -> Pos2;
}

struct Node {
    id: i8,
    name: String,
    inputs: Vec<InPort>,
    outputs: Vec<OutPort>
}

struct Node_RealNumber {
    node : Node,
    
}