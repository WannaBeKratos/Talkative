use{
    super::structs::Talkative
};

impl Talkative {
    pub fn add_GlobalContext(&mut self,context:&str)->(){
        self.globalContext = context.to_string();
    }
}