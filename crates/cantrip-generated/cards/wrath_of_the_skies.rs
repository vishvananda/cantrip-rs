// Wrath of the Skies
// status: ready
// You get X {E} (energy counters), then you may pay any amount of {E}. Destroy each artifact, creature, and enchantment with mana value less than or equal to the amount of {E} paid this way.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (i64::from(item.x)).max(0) as i32 }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.change_energy(player_0, amount_0);
g.end_sentence();
let mut done_1: i64 = 0; for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { done_1 += { let mut paid = 0i64; while g.state().player(player).energy() > 0 && cantrip_mechanics::effects::ask_may(g, player, asker.source, Purpose::Optional)? { g.change_energy(player, -1); paid += 1; } paid }; }
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment)) && (i64::from(q.mana_value(id))) <= ((done_1 as i64)))).collect::<Vec<ObjectId>>() }; let destroyed_2 = g.destroy_all(&objects_2);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Wrath of the Skies", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
