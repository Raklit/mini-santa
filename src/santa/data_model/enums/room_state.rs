use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum RoomState {
    ChosingAGift = 0,
    BuyingAGift = 1,
    MailerAwaitingGiftDelivery = 2,
    MailerSendGiftToRecipient = 3,
    GiftHasBeenDelivered = 4,
    RecipientTookTheGift = 5
}