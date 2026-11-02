// Zhao, the Moon Slayer
// status: ready
// Menace
// Nonbasic lands enter tapped.
// {7}: Put a conqueror counter on Zhao.
// As long as Zhao has a conqueror counter on him, nonbasic lands are Mountains. (They lose all other land types and abilities and have "{T}: Add {R}.")
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::ZoneChange(z) = proposal { let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if z.to.is(ZoneKind::Battlefield) && (!(q.chars(id).supertypes.basic) && q.chars(id).is(CardType::Land)) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::ZoneChange(seen) = &seen { let z = seen; let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if let Proposal::ZoneChange(z) = proposal { z.tapped = true; } } }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_0.push((object, CounterKind::Conqueror, amount)); } }
let had_0: Vec<_> = counters_0.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_0); let applied_0: Vec<ObjectId> = counters_0.iter().zip(had_0).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.source).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::Conqueror))) > (0i64))) && ((!((*so_far).supertypes.basic) && (*so_far).is(CardType::Land))) }
#[allow(unused_variables)] fn ability_3_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.set_land_subtypes(SubtypeSet::NONE.with(q.words(asker.source).subtype(Subtype::Mountain))); }

pub static CARD: CardDef = CardDef { name: "Zhao, the Moon Slayer", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 7, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_2, "{7}: Put a conqueror counter on Zhao.").targets(&[])], statics: &[sets_land_subtypes(ability_3_0_affects, ability_3_0_apply_0, "As long as Zhao has a conqueror counter on him, nonbasic lands are Mountains.")], replacements: &[replacement(ReplacementKind::ZoneChange, ability_1_applies, ability_1_apply, "Nonbasic lands enter tapped.")], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
