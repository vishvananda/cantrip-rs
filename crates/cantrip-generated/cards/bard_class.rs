// Bard Class
// status: ready
// (Gain the next level as a sorcery to add its ability.)
// Legendary creatures you control enter with an additional +1/+1 counter on them.
// {R}{G}: Level 2
// Legendary spells you cast cost {R}{G} less to cast. This effect reduces only the amount of colored mana you pay.
// {3}{R}{G}: Level 3
// Whenever you cast a legendary spell, exile the top two cards of your library. You may play them this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::ZoneChange(z) = proposal { let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if z.to.is(ZoneKind::Battlefield) && (q.chars(id).supertypes.legendary && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(z.controller)) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::ZoneChange(seen) = &seen { let z = seen; let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if let Proposal::ZoneChange(z) = proposal { z.counters[CounterKind::PlusOne as usize] = z.counters[CounterKind::PlusOne as usize].saturating_add((1i64).max(0) as i16); } } }
#[allow(unused_variables)] fn ability_1_level_2_0_0_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if (q.state().obj(asker.source).class_level >= 2) && player == asker.controller && q.chars(spell).supertypes.legendary { { let mut less = ManaCost::ZERO; less.symbols[3] += (1i64).max(0) as u32; less.symbols[4] += (1i64).max(0) as u32; CostDelta::less_mana(less) } } else { CostDelta::NONE } }
#[allow(unused_variables)] fn ability_1_level_3_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && q.chars(id).supertypes.legendary && q.state().obj(asker.source).class_level >= 3 { return Some(Fired { subject, data }); } }
 None }
fn ability_1_level_3_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((2i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
cantrip_api::instruction(g, |g| { let cards_1_0 = { let q: &dyn Query = g; moved_0.clone() }; let players_1_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; cantrip_api::instruction(g, |g| { for &card in &cards_1_0 { if g.obj(card).zone.is(ZoneKind::Exile) { for &player in &players_1_0 { g.allow_play(card, player, PlayGrant { lands: true, expiry: Expiry::EndOfTurn, ..PlayGrant::CAST }); } } } });
 });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Bard Class", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Class]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::class::level_two("{R}{G}", "{R}{G}: Level 2"),cantrip_mechanics::class::level_three("{3}{R}{G}", "{3}{R}{G}: Level 3"),triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_1_level_3_0_event), ability_1_level_3_0, "Legendary spells you cast cost {R}{G} less to cast. This effect reduces only the amount of colored mana you pay.").targets(&[])], statics: &[], replacements: &[replacement(ReplacementKind::ZoneChange, ability_0_applies, ability_0_apply, "Legendary creatures you control enter with an additional +1/+1 counter on them.")], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cost_changes: &[ability_1_level_2_0_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
