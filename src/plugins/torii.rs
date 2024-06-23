use crate::torii::run_torii_client;
use bevy::prelude::*;
use starknet_ff::FieldElement;
use tokio::runtime::Builder;
use torii_grpc::types::schema::Entity as ToriiData;

pub struct ToriiPlugin;
impl Plugin for ToriiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_tokio_and_torii)
            .add_systems(Update, update_torii_data);
    }
}

#[derive(Resource)]
struct ToriiResource {
    entity: ToriiData,
    rx: tokio::sync::mpsc::Receiver<ToriiData>,
}

fn setup_tokio_and_torii(mut commands: Commands) {
    // run torii client in separate thread via tokio
    let tokio_runtime = Builder::new_current_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    let (tx, rx) = tokio::sync::mpsc::channel::<ToriiData>(16);

    std::thread::spawn(move || {
        tokio_runtime.block_on(run_torii_client(tx));
    });

    let default_entity = ToriiData {
        hashed_keys: FieldElement::default(),
        models: Vec::new(),
    };

    commands.insert_resource(ToriiResource {
        entity: default_entity,
        rx,
    });
}

fn update_torii_data(mut torii_entity: ResMut<ToriiResource>) {
    if let Ok(new_entity) = torii_entity.rx.try_recv() {
        info!("Message from Torii Client: {:?}", new_entity);
        torii_entity.entity = new_entity;
    }
}
