mod module_a {
    pub fn public_function() {
        println!("This is a public function in module A");
    }
    
    fn private_function() {
        println!("This is a private function in module A");
    }
    
    pub fn call_private_function() {
        private_function(); // 在同一模块内可以访问private_function
    }
}

mod module_b {
    pub fn some_function() {
        super::module_a::public_function(); // 可以访问其他模块的公共函数
        // super::module_a::private_function(); // 这行会出错，因为private_function不可见
    }
}

fn main() {
    module_a::public_function();
    module_a::call_private_function();
    module_b::some_function();
}