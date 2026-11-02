// Ulalek, Fused Atrocity
// status: planned
// requires: copies of each of several abilities on the stack
// Devoid (This card has no color.)
// Whenever you cast an Eldrazi spell, you may pay {C}{C}. If you do, copy all spells you control, then copy all other activated and triggered abilities you control. You may choose new targets for the copies. (Mana abilities can't be copied.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.colors = ColorSet::NONE; }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Eldrazi)) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_0 = g.may_pay(payer_0, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 2], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &PAY_QUESTION)?;
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut open_1: Vec<ObjectId> = { let q: &dyn Query = g; q.state().stack.iter().filter(|i| i.kind == StackKind::Spell).map(|i| i.object).filter(|&id| (q.state().stack.iter().find(|i| i.object == id).is_some_and(|i| i.kind == StackKind::Spell) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; while let Some(&first) = open_1.first() { let spell = if open_1.iter().all(|&o| o == first) { Some(first) } else { let mut distinct = open_1.clone(); distinct.sort(); distinct.dedup(); cantrip_mechanics::choose::card(g, player_1, asker.source, Purpose::CopyWhich, distinct)? }; let Some(spell) = spell else { break; }; if let Some(at) = open_1.iter().position(|&o| o == spell) { open_1.remove(at); } g.copy_spell(spell, player_1, true)?; }
g.end_sentence();
planned!("copies-of-each-of-several-abilities-on-the-stack", CopyEach, "actor" = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, "objects" = "{\"args\":[{\"args\":{\"args\":\"ThisAbility\",\"kind\":\"OnStack\"},\"kind\":\"Not\"},{\"args\":[{\"args\":\"Activated\",\"kind\":\"OnStack\"},{\"args\":\"Triggered\",\"kind\":\"OnStack\"}],\"kind\":\"Either\"},{\"args\":{\"args\":{\"kind\":\"AbilityController\"},\"kind\":\"One\"},\"kind\":\"ControlledBy\"}],\"kind\":\"All\"}", "targets" = "\"May\"");
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Ulalek, Fused Atrocity", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::C, Mana::W)),Some((Mana::C, Mana::U)),Some((Mana::C, Mana::B)),Some((Mana::C, Mana::R)),Some((Mana::C, Mana::G)),None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Eldrazi]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_1_event), ability_1, "Whenever you cast an Eldrazi spell, you may pay {C}{C}. If you do, copy all spells you control, then copy all other activated and triggered abilities you control. You may choose new targets for the copies.").targets(&[])], statics: &[characteristic_defining(Layer::Color, ability_0_apply_0, "Devoid")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
