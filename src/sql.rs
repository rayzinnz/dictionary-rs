use anyhow::Result;
use turso::Builder;

const DB_PATH:&str = r"thesaurus.db";

#[derive(Debug, Clone, Default)]
pub struct Word {
	pub words_rid: i64,
	pub word: String,
	pub definitions: Vec<Definition>,
	pub synonyms: Vec<String>,
	pub antonyms: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct WordShort {
	pub words_rid: i64,
	pub word: String,
}

#[derive(Debug, Clone)]
pub struct Definition {
	pub definition: String,
	pub sub_senses: Vec<String>,
}

pub fn get_db_path() -> String {
    let exe_path = std::env::current_exe().expect("Could not get exe_path for db file");

    let exe_dir = exe_path
        .parent()
        .expect("Executable should have a parent directory");

    exe_dir.join(DB_PATH).to_string_lossy().to_string()
}

pub async fn get_words(search: &str) -> Result<Vec<WordShort>> {
	let mut rtn:Vec<WordShort> = Vec::new();

    let sql = format!(r"
SELECT
w.rid
,w.word
FROM words w
WHERE w.word LIKE '{search}%'
;
	");

	let db = Builder::new_local(&get_db_path()).build().await?;
	let conn = db.connect()?;
	let mut rows = conn.query(sql, ()).await?;
	
    while let Some(row) = rows.next().await? {
        let words_rid = row.get_value(0)?;
		let words_rid = words_rid.as_integer().expect("Did not expect NULL");
        let word = row.get_value(1)?;
		let word = word.as_text().expect("Did not expect NULL").to_string();
		rtn.push(WordShort {words_rid: *words_rid, word: word});
    }

	// println!("words: {:?}", rtn);

	Ok(rtn)
}

pub async fn get_word(words_rid: i64) -> Result<Option<Word>> {
	let word: String;
	let mut definitions: Vec<Definition> = Vec::new();
	let mut synonyms: Vec<String> = Vec::new();
	let mut antonyms: Vec<String> = Vec::new();

	let db = Builder::new_local(&get_db_path()).build().await?;
	let conn = db.connect()?;
	
    let sql = format!(r"
SELECT
w.word
FROM words w
WHERE w.rid = {words_rid}
;
	");
	let mut rows = conn.query(sql, ()).await?;
	match rows.next().await {
		Ok(row) => {
			word = row.expect("Did not expect NULL 1").get_value(0)?.as_text().expect("Did not expect NULL 2").to_string();
		}
		Err(e) => panic!("Error getting words_rid {words_rid} from db: {e}")
	}

    let sql = format!(r"
SELECT
	wl.linktype
	,wl.word
FROM wordlinks wl
WHERE wl.words_rid = {words_rid}
ORDER BY wl.rid ASC
;
	");
	let mut rows = conn.query(sql, ()).await?;
    while let Some(row) = rows.next().await? {
		let link_type = row.get_value(0)?;
		let link_type = link_type.as_integer().expect("Did not expect NULL");
        let linked_word = row.get_value(1)?.as_text().expect("Did not expect NULL").to_string();
		//--0 = synonym, 1 = antonym
		if *link_type==0 {
			synonyms.push(linked_word);
		} else {
			antonyms.push(linked_word);
		}
    }

    let sql = format!(r"
SELECT
	d.definitiontype
	,d.definition
FROM definitions d
WHERE d.words_rid = {words_rid}
ORDER BY d.rid ASC
;
	");
	let mut rows = conn.query(sql, ()).await?;
	let mut definition: Definition = Definition { definition: String::new(), sub_senses: Vec::new() };
	let mut subsenses: Vec<String> = Vec::new();
    while let Some(row) = rows.next().await? {
		let definitiontype = row.get_value(0)?;
		let definitiontype = definitiontype.as_integer().expect("Did not expect NULL");
        let definition_str = row.get_value(1)?.as_text().expect("Did not expect NULL").to_string();
		//--0 = sense, 1 = subsense
		if *definitiontype==0 {
			if !definition.definition.is_empty() {
				definition.sub_senses = subsenses.clone();
				definitions.push(definition.clone());
			}
			definition = Definition { definition: definition_str, sub_senses: Vec::new() };
			subsenses = Vec::new();
		} else {
			subsenses.push(definition_str);
		}
    }
	if !definition.definition.is_empty() {
		definition.sub_senses = subsenses.clone();
		definitions.push(definition.clone());
	}

	// println!("words: {:?}", rtn);

	Ok(Some(Word {
		words_rid,
		word,
		definitions,
		synonyms,
		antonyms,
	}))
}
