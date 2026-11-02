// Sunspine Lynx
// status: ready
// Players can't gain life.
// Damage can't be prevented.
// When this creature enters, it deals damage to each player equal to the number of nonbasic lands that player controls.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true }
#[allow(unused_variables)] fn ability_1_unpreventable(q: &dyn Query, asker: &Asker, source: ObjectId, combat: bool) -> bool { true }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_1: Vec<(ObjectId, Target, u16)> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let source_2 = { let q: &dyn Query = g; item.subject.object() }; let amount_2 = { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).supertypes.basic) && q.chars(id).is(CardType::Land) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64).min(u16::MAX.into()) as u16 }; if let Some(source) = source_2 { for t in { let q: &dyn Query = g; (Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() } { gathered_1.push((source, t, amount_2)); } }
    } 
g.damage_together(&gathered_1);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Sunspine Lynx", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental,Subtype::Cat]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "When this creature enters, it deals damage to each player equal to the number of nonbasic lands that player controls.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cant_gain_life: &[ability_0_0_rule], unpreventable_damage: &[ability_1_unpreventable], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
