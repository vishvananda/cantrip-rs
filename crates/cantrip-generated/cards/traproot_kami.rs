// Traproot Kami
// status: ready
// Defender; reach (This creature can block creatures with flying.)
// Traproot Kami's toughness is equal to the number of Forests on the battlefield.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.toughness = Some(((q.state().battlefield().iter().copied().filter(|&id| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Forest))).collect::<Vec<ObjectId>>()).len() as i64).clamp(-32768, 32767) as i16); }

pub static CARD: CardDef = CardDef { name: "Traproot Kami", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spirit]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Defender,Keyword::Reach]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[characteristic_defining(Layer::PtDefine, ability_2_apply_0, "Traproot Kami's toughness is equal to the number of Forests on the battlefield.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
