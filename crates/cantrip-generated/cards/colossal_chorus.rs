// Colossal Chorus
// status: planned
// requires: engine: no StartingIntensity
// requires: intensity
// requires: operation Conjure
// requires: subtype Chorus
// Starting intensity 2
// Conjure X cards named Colossal Dreadmaw onto the battlefield, where X is this spell's intensity.
// All Chorus cards you own intensify.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let count_0: u32 = { let q: &dyn Query = g; (Some(asker.source).map_or(0i64, |o: ObjectId| planned_value!("intensity", 0i64 => i64::from(q.state().obj(o).intensity)))).max(0) as u32 }; let mut conjured_0: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_0.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::Named("Colossal Dreadmaw"), count_0, ConjureTo::Battlefield { tapped: false })?)); }
g.end_sentence();
planned!("subtype-chorus" => cantrip_mechanics::intensity::intensify_all(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, |q: &dyn Query, asker: &Asker, id: ObjectId| -> bool { q.chars(id).has_subtype(Subtype::Chorus) }));
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-starting-intensity" => enters: cantrip_mechanics::intensity::starting::<2>);

pub static CARD: CardDef = CardDef { name: "Colossal Chorus", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[planned_value!("subtype Chorus", Subtype::Plains => Subtype::Chorus)]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
