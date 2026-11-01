// Yuan-Ti Malison
// status: planned
// requires: engine: venture into the dungeon reaches only Undercity
// This creature can't be blocked as long as it's attacking alone.
// Whenever this creature deals combat damage to a player, venture into the dungeon. (Enter the first room or advance to the next room.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.source).is_some_and(|candidate: ObjectId| (q.state().attackers.len() == 1 && q.state().attackers.iter().any(|a| a.id == candidate)))) && (source == affected) }
#[allow(unused_variables)] fn ability_0_0_0_rule(q: &dyn Query, source: ObjectId, blocker: ObjectId, attacker: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); !((Some(asker.source).is_some_and(|candidate: ObjectId| (q.state().attackers.len() == 1 && q.state().attackers.iter().any(|a| a.id == candidate)))) && attacker == asker.source && true) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("engine-venture-into-the-dungeon-reaches-only-undercity" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.venture(player)?; });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Yuan-Ti Malison", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Snake,Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_1_event), ability_1, "Whenever this creature deals combat damage to a player, venture into the dungeon.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[ability_0_0_0_rule], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
