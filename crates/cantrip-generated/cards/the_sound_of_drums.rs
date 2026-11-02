// The Sound of Drums
// status: planned
// requires: static rule GoadedBy
// Enchant creature
// Enchanted creature is goaded.
// If enchanted creature would deal combat damage to a permanent or player, it deals double that damage instead.
// {2}{R}: Return this card from your graveyard to your hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.state().obj(asker.source).attached_to == Some(id) }
#[allow(unused_variables)] fn ability_2_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::Damage(e) = proposal { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to) == Some(source) && match *target { Target::Object(o) => true, Target::Player(p) => true, _ => false } { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_2_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::Damage(seen) = &seen { let e = seen; let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if let Proposal::Damage(e) = proposal { e.amount = ((2i64 * i64::from(data[0]))).min(u16::MAX.into()) as u16; } } }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_0 = g.return_to_hand(&objects_0);
g.end_sentence();
 Ok(()) }
planned_item!("static-rule-goaded-by" => goaded_rules: ability_1_0_rule);

pub static CARD: CardDef = CardDef { name: "The Sound of Drums", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_3, "{2}{R}: Return this card from your graveyard to your hand.").targets(&[]).from(ZoneKind::Graveyard)], statics: &[], replacements: &[replacement(ReplacementKind::Damage, ability_2_applies, ability_2_apply, "If enchanted creature would deal combat damage to a permanent or player, it deals double that damage instead.")], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
