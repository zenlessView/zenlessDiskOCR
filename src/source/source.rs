use {
  crate::source::word_groups,
  std::collections::{HashMap, HashSet},
  tokio::sync::OnceCell
};

pub static DISK_SETS: &'static [&str; 16] = &[
  "折枝剑歌",
  "静听嘉音",
  "混沌爵士",
  "原始朋克",
  "啄木鸟电音",
  "河豚电音",
  "震星迪斯科",
  "自由蓝调",
  "激素朋克",
  "灵魂摇滚",
  "摇摆爵士",
  "炎狱重金属",
  "混沌重金属",
  "雷暴重金属",
  "极地重金属",
  "獠牙重金属"
];

pub static DISK_AFFIX_NAMES: &'static [&str; 16] = &[
  "生命值",
  "攻击力",
  "防御力",
  "暴击率",
  "暴击伤害",
  "异常精通",
  "穿透率",
  "物理伤害加成",
  "火属性伤害加成",
  "冰属性伤害加成",
  "电属性伤害加成",
  "以太伤害加成",
  "异常掌控",
  "冲击力",
  "能量自动回复",
  "穿透值"
];

static FIXED_WORDS: &'static [&str; 2] = &["主属性", "副属性"];

static POSSIBLE_WORDS: OnceCell<Vec<Vec<&'static str>>> = OnceCell::const_new();

static CLOSE_WORD_GROUPS: OnceCell<Vec<Vec<&'static str>>> = OnceCell::const_new();

static CLOSE_WORDS: OnceCell<HashMap<&'static str, HashSet<&'static str>>> =
  OnceCell::const_new();

pub async fn get_possible_words() -> &'static Vec<Vec<&'static str>> {
  POSSIBLE_WORDS
    .get_or_init(|| {
      async {
        let mut result: Vec<Vec<&'static str>> = vec![vec![]; 8];

        let possible_words =
          DISK_SETS.iter().chain(DISK_AFFIX_NAMES.iter()).chain(FIXED_WORDS.iter());

        for word in possible_words {
          let length = word.chars().count();
          if length <= 8 {
            result[length - 1].push(word);
          }
        }

        result
      }
    })
    .await
}

pub async fn get_close_word_groups() -> &'static Vec<Vec<&'static str>> {
  CLOSE_WORD_GROUPS
    .get_or_init(|| {
      async {
        let mut result = vec![];

        let possible_words = get_possible_words().await;

        for pool in possible_words.iter() {
          result.extend(word_groups::get_close_word_groups(pool));
        }

        result
      }
    })
    .await
}

pub async fn get_close_words() -> &'static HashMap<&'static str, HashSet<&'static str>> {
  CLOSE_WORDS
    .get_or_init(|| {
      async {
        let mut result = HashMap::new();

        for word in get_possible_words().await.iter().flatten() {
          result.insert(*word, HashSet::new());
        }

        let close_word_groups = get_close_word_groups().await;

        for group in close_word_groups.iter() {
          for word in group.iter() {
            result.get_mut(*word).unwrap().extend(group.iter().filter(|&w| w != word));
          }
        }

        result
      }
    })
    .await
}
