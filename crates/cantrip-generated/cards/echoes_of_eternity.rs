// Echoes of Eternity
// status: ready
// If a triggered ability of a colorless spell you control or another colorless permanent you control triggers, that ability triggers an additional time.
// Whenever you cast a colorless spell, copy it. You may choose new targets for the copy. (A copy of a permanent spell becomes a token.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_trigger_rule(q: &dyn Query, asker: &Asker, event: &Event, triggering: &Triggering) -> bool { ((q.state().obj(triggering.source).zone.is(ZoneKind::Stack) && (q.chars(triggering.source).colors.is_empty() && Some(asker.controller) == Some(triggering.controller))) || (triggering.permanent && (!(Some(asker.source) == Some(triggering.source)) && q.chars(triggering.source).colors.is_empty() && (q.chars(triggering.source).is(CardType::Artifact) || q.chars(triggering.source).is(CardType::Battle) || q.chars(triggering.source).is(CardType::Creature) || q.chars(triggering.source).is(CardType::Enchantment) || q.chars(triggering.source).is(CardType::Land) || q.chars(triggering.source).is(CardType::Planeswalker)) && Some(asker.controller) == Some(triggering.controller)))) && true }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && q.chars(id).colors.is_empty() { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let copier_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let spell_0: Option<ObjectId> = ({ let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()) }).first().copied();
if let Some(spell) = spell_0 { g.copy_spell(spell, copier_0, true)?; }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Echoes of Eternity", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 3], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Kindred,CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Eldrazi]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_1_event), ability_1, "Whenever you cast a colorless spell, copy it. You may choose new targets for the copy.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, additional_triggers: &[ability_0_trigger_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
