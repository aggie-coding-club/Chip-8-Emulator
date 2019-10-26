trait CPUOperationAppliable {
    fn clear(&mut self);
    fn ret(&mut self);
    fn jump(&mut self);
    fn call(&mut self);
    fn skip_equal_byte(&mut self);
    fn skip_neq_byte(&mut self);
    fn skip_equal(&mut self);
    fn load_byte(&mut self);
    fn add_byte(&mut self);
    fn load(&mut self);
    fn or(&mut self);
    fn and(&mut self);
    fn xor(&mut self);
    fn add(&mut self);
    fn sub(&mut self);
    fn shr(&mut self);
    fn subn(&mut self);
    fn shl(&mut self);
    fn skip_neq(&mut self);
    fn load_index_byte(&mut self);
    fn jump_r0_byte(&mut self);
    fn rand(&mut self);
    fn draw(&mut self);
    fn skip_key(&mut self);
    fn skip_neq_key(&mut self);
    fn load_reg_delay(&mut self);
    fn load_reg_key(&mut self);
    fn load_delay_reg(&mut self);
    fn load_sound_reg(&mut self);
    fn add_i_reg(&mut self);
    fn load_sprite(&mut self);
    fn load_bcd(&mut self);
    fn load_mem_registers(&mut self);
    fn load_registers_mem(&mut self);
}
