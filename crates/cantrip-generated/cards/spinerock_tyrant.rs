// Spinerock Tyrant
// status: planned
// requires: operation EffectOn
// Flying
// Wither (This deals damage to creatures in the form of -1/-1 counters.)
// Whenever you cast an instant or sorcery spell with a single target, you may copy it. If you do, those spells gain wither. You may choose new targets for the copy.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && ((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) && q.state().stack.iter().find(|i| i.object == id).is_some_and(|i| q.state().targets(i).len() == 1)) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_0 = cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)?; if paid_0 {
let copier_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let spell_1: Option<ObjectId> = ({ let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()) }).first().copied(); let mut copies_1: Vec<ObjectId> = Vec::new();
if let Some(spell) = spell_1 { copies_1.extend(g.copy_spell(spell, copier_1, true)?); }
g.end_sentence();
hoisted_2.extend(copies_1.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
planned!("operation-effect-on" => cantrip_mechanics::effects::effect_on(g, asker, EffectOn::EachSpell(|q: &dyn Query, asker: &Asker, id: ObjectId| (item.subject.object() == Some(id) || (hoisted_2.clone()).first().copied() == Some(id))), &[SpellEffect::Wither], Some(Duration::Indefinite))?);
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Spinerock Tyrant", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Wither]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_2_event), ability_2, "Whenever you cast an instant or sorcery spell with a single target, you may copy it. If you do, those spells gain wither. You may choose new targets for the copy.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
