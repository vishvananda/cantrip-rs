// Teferi, Mage of Zhalfir
// status: ready
// Flash
// Creature cards you own that aren't on the battlefield have flash.
// Each opponent can cast spells only any time they could cast a sorcery.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.state().obj(affected).owner)) && { let o = q.state().obj(affected); !o.zone.is(ZoneKind::Battlefield) && !o.token && !o.card_copy } }
#[allow(unused_variables)] fn ability_1_0_flash(q: &dyn Query, asker: &Asker, spell: ObjectId, player: PlayerId) -> bool { q.chars(spell).keywords.has(Keyword::Flash) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Flash); }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, from: ZoneKind) -> bool { let s = q.state(); !(player != asker.controller) || (player == s.turn.active && s.turn.step().is_main() && s.stack.is_empty()) }

pub static CARD: CardDef = CardDef { name: "Teferi, Mage of Zhalfir", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 3, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_1_affects, ability_1_apply_0, "Creature cards you own that aren't on the battlefield have flash.").everywhere()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cast_rules: &[ability_2_0_rule], flash_rules: &[ability_1_0_flash], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
