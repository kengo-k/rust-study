#[cfg(test)]
mod tests {

    #[test]
    fn test_vec() {
      let mut v = Vec::new();
      v.push(5);
      v.push(6);
      v.push(7);
      v.push(8);
      println!("v: {:?}", v);
    }

}
