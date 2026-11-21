pub trait WithNewId: Clone {
    fn id_mut(&mut self) -> &mut Option<String>;

    fn with_new_id(&self) -> (Self, String) {
        let mut data = self.clone();
        let id = uuid::Uuid::now_v7().to_string();

        *data.id_mut() = Some(id.clone());

        (data, id)
    }
}
