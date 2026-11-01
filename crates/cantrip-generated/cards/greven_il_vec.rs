// Greven il-Vec
// status: planned
// requires: card type Vanguard
// requires: layout Vanguard
// Whenever a creature you control deals damage to a creature, destroy the other creature. It can't be regenerated.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Object(o) => o.0, _ => u32::MAX }]; if !q.state().obj(source).zone.is(ZoneKind::Stack) && (q.chars(source).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(source))) && matches!(*target, Target::Object(o) if q.chars(o).is(CardType::Creature)) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; Some(ObjectId(item.data[1])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>() }; let destroyed_0 = g.destroy_all_no_regen(&objects_0);
g.end_sentence();
 Ok(()) }
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: -1, life: 2 }));

pub static CARD: CardDef = CardDef { name: "Greven il-Vec", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_0_event), ability_0, "Whenever a creature you control deals damage to a creature, destroy the other creature. It can't be regenerated.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
