// Khorvath Brightflame
// status: ready
// Partner with Sylvia Brightspear (When this creature enters, target player may put Sylvia into their hand from their library, then shuffle.)
// Flying, haste
// Knights your team controls have flying and haste.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_3_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).has_subtype(q.words(asker.source).subtype(Subtype::Knight)) && (q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0) == Some(asker.controller)).collect::<Vec<PlayerId>>()).contains(&q.controller(affected))) }
#[allow(unused_variables)] fn ability_3_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Flying);
chars.keywords = chars.keywords.with(Keyword::Haste); }

pub static CARD: CardDef = CardDef { name: "Khorvath Brightflame", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_3_affects, ability_3_apply_0, "Knights your team controls have flying and haste.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
