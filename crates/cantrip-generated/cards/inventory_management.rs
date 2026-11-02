// Inventory Management
// status: ready
// Split second (As long as this spell is on the stack, players can't cast spells or activate abilities that aren't mana abilities.)
// For each Aura and Equipment you control, you may attach it to a creature you control.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let attachments_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Aura)) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Equipment))) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let hosts_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; for &attachment in &attachments_0 { let chosen = cantrip_mechanics::choose::select(g, player_0, asker.source, Purpose::Attach, hosts_0.clone(), cantrip_mechanics::select::Selection::of(cantrip_mechanics::select::up_to(1), &[]))?; if let Some(&host) = chosen.first() { g.attach(attachment, host); } }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Inventory Management", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::SplitSecond]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
