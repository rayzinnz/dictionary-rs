use anyhow::Result;
use turso::Builder;

// #[cfg(target_os = "linux")]
// const DB_PATH:&str = "/home/ray/MEGA/Rays/Programming/english/thesaurus.db";
// #[cfg(target_os = "windows")]
// const DB_PATH:&str = r"C:\Users\hrag\Sync\Programming\english\thesaurus.db";

const DB_PATH:&str = r"thesaurus.db";

#[derive(Debug, Clone)]
pub struct Word {
	pub word_rid: i64,
	pub word: String,
	pub definitions: Vec<String>,
	pub synonyms: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct WordShort {
	pub word_rid: i64,
	pub word: String,
}

pub async fn get_words(search: &str) -> Result<Vec<String>> {
	let mut rtn:Vec<String> = Vec::new();

    let sql = format!(r"
SELECT
w.word
FROM words w
WHERE w.word LIKE '{search}%'
;
	");

	let db = Builder::new_local(DB_PATH).build().await?;
	let conn = db.connect()?;
	let mut rows = conn.query(sql, ()).await?;
	
    while let Some(row) = rows.next().await? {
        let word = row.get_value(0)?;
		let word = word.as_text().unwrap().to_string();
		rtn.push(word);
    }

	// println!("words: {:?}", rtn);

	Ok(rtn)
}

pub async fn get_word(search: &str) -> Result<Vec<WordShort>> {
	let mut rtn:Vec<WordShort> = Vec::new();

    let sql = format!(r"
SELECT
w.rid
,w.word
FROM words w
WHERE w.word LIKE '{search}%'
;
	");

	let db = Builder::new_local(DB_PATH).build().await?;
	let conn = db.connect()?;
	let mut rows = conn.query(sql, ()).await?;
	
    while let Some(row) = rows.next().await? {
        let word_rid = row.get_value(0)?;
		let word_rid = word_rid.as_integer().unwrap();
        let word = row.get_value(1)?;
		let word = word.as_text().unwrap().to_string();
		rtn.push(WordShort {word_rid: *word_rid, word: word});
    }

	// println!("words: {:?}", rtn);

	Ok(rtn)
}
