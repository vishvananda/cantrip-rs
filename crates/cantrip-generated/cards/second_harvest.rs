// Second Harvest
// status: ready
// For each token you control, create a token that's a copy of that permanent.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let originals_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.state().obj(id).token && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let player_0 = { let q: &dyn Query = g; Some(asker.controller) }; let created_0 = if let Some(player) = player_0 { let copies: Vec<(Copiable, PlayerId)> = originals_0.iter().map(|&original| (g.copiable_values(original), player)).collect(); g.create_token_copies_together(&copies)? } else { Vec::new() };
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Second Harvest", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
