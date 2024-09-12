
#[derive(Debug)]
pub struct SerRope {
    chunks: Vec<String>,
    indicies: Vec<u32>,
}

impl std::fmt::Display for SerRope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &idx in &self.indicies {
            f.write_str(&self.chunks[idx as usize])?;
        }
        Ok(())
    }
}

impl From<ropey::Rope> for SerRope {
    fn from(value: ropey::Rope) -> Self {
        let mut seen = ahash::AHashMap::new();
        let mut chunks = Vec::new();
        let mut indicies = Vec::new();
        for chunk in value.chunks() {
            if let Some(idx) = seen.get(chunk) {
                indicies.push(*idx);
            } else {
                let idx = chunks.len() as u32;
                chunks.push(chunk.to_string());
                indicies.push(idx);
                seen.insert(chunk, idx);
            }
        }
        Self { chunks, indicies }
    }
}

impl From<im_rope::Rope> for SerRope {
    fn from(value: im_rope::Rope) -> Self {
        let mut seen = ahash::AHashMap::new();
        let mut chunks = Vec::new();
        let mut indicies = Vec::new();
        for chunk in value.chunks() {
            if let Some(idx) = seen.get(&chunk) {
                indicies.push(*idx);
            } else {
                let idx = chunks.len() as u32;
                let s = match chunk {
                    im_rope::Chunk::Char(c) => c.to_string(),
                    im_rope::Chunk::Str(s) => s.to_string(),
                };
                chunks.push(s);
                indicies.push(idx);
                seen.insert(chunk, idx);
            }
        }
        Self { chunks, indicies }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use im_rope::Rope;
    use ropey::RopeBuilder;


    #[test]
    fn round_trip_ropey() {
        let pool = [
            "hello",
            "thing",
            "stuff",
            "yes",
            "no",
        ];
        let mut rb = RopeBuilder::new();
        for i in 0..100 {
            rb.append(pool[i % pool.len()]);
            if i % 10 == 0 {
                rb.append(" ")
            }
        }
        let sr: SerRope = rb.finish().into();
        insta::assert_debug_snapshot!(sr);
        insta::assert_snapshot!(sr.to_string());
    }

    #[test]
    fn round_trip_im_rope() {
        let pool = [
            "hello",
            "thing",
            "stuff",
            "yes",
            "no",
        ];
        let mut rb = Rope::new();
        for i in 0..100 {
            rb.append(pool[i % pool.len()]);
            if i % 10 == 0 {
                rb.append(" ")
            }
        }
        let sr: SerRope = rb.into();
        insta::assert_debug_snapshot!(sr);
        insta::assert_snapshot!(sr.to_string());
    }
}
