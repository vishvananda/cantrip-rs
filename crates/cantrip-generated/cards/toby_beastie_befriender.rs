// Toby, Beastie Befriender
// status: planned
// requires: static rule CombatGroup Attacker
// requires: static rule CombatGroup Blocker
// When Toby enters, create a 4/4 white Beast creature token with "This token can't attack or block alone."
// As long as you control four or more creature tokens, creature tokens you control have flying.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { id == asker.source }
#[allow(unused_variables)] fn ability_0_0_rule_counted(q: &dyn Query, asker: &Asker, id: ObjectId, other: ObjectId) -> bool { true }
#[allow(unused_variables)] fn ability_0_1_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { id == asker.source }
#[allow(unused_variables)] fn ability_0_1_rule_counted(q: &dyn Query, asker: &Asker, id: ObjectId, other: ObjectId) -> bool { true }
planned_item!("static-rule-combat-group-attacker" => combat_groups: CombatGroupRule { role: CombatRole::Attacker, applies: ability_0_0_rule, counted: ability_0_0_rule_counted, others_only: false, count: 2..=u32::MAX });
planned_item!("static-rule-combat-group-blocker" => combat_groups: CombatGroupRule { role: CombatRole::Blocker, applies: ability_0_1_rule, counted: ability_0_1_rule_counted, others_only: false, count: 2..=u32::MAX });

pub static CARD: CardDef = CardDef { name: token!(Beast).name, token_words: token!(Beast).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Beast]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (((q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).is(CardType::Creature) && q.state().obj(id).token) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) >= (4i64)) && (((*so_far).is(CardType::Creature) && q.state().obj(affected).token && Some(asker.controller) == Some(q.controller(affected)))) }
#[allow(unused_variables)] fn ability_1_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Flying); }

pub static CARD: CardDef = CardDef { name: "Toby, Beastie Befriender", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Toby enters, create a 4/4 white Beast creature token with \"This token can't attack or block alone.\"").targets(&[])], statics: &[static_ability(Layer::Ability, ability_1_0_affects, ability_1_0_apply_0, "As long as you control four or more creature tokens, creature tokens you control have flying.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
