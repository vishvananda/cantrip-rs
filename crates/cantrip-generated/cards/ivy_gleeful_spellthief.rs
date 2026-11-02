// Ivy, Gleeful Spellthief
// status: ready
// Flying
// Whenever a player casts a spell that targets only a single creature other than Ivy, you may copy that spell. The copy targets Ivy. (A copy of an Aura spell becomes a token.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if true && (q.state().stack.iter().find(|i| i.object == id).is_some_and(|i| { let chosen = q.state().targets(i); !chosen.is_empty() && chosen.iter().all(|t| *t == chosen[0]) }) && q.state().stack.iter().find(|i| i.kind == StackKind::Spell && i.object == id).is_some_and(|i| q.state().targets(i).iter().any(|t| t.object().is_some_and(|o| (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && !(Some(asker.source) == Some(id)))).collect::<Vec<ObjectId>>()).contains(&o))))) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let copier_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let spell_1 = { let q: &dyn Query = g; item.subject.object() }; let to_1: Vec<ObjectId> = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; if let Some(spell) = spell_1 { if let Some(copy) = g.copy_spell(spell, copier_1, false)? { if let Some(&to) = to_1.first() { let count = { let q: &dyn Query = g; q.state().stack.iter().find(|i| i.kind == StackKind::Spell && i.object == copy).map_or(0, |i| q.state().targets(i).len()) }; for _ in 0..count { g.change_a_target_to(Target::Object(copy), to, copier_1)?; } } } }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Ivy, Gleeful Spellthief", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Faerie,Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_1_event), ability_1, "Whenever a player casts a spell that targets only a single creature other than Ivy, you may copy that spell. The copy targets Ivy.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
