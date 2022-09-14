import React, { useEffect, useState } from "react";
import { Form, Grid } from "semantic-ui-react";

import { useSubstrateState } from "./substrate-lib";
import { TxButton } from "./substrate-lib/components";

import KittyCards from "./KittyCards";

export default function Kitties(props) {
  /* Get substrate context */
  const { api, keyring } = useSubstrateState();

  const [status, setStatus] = useState("");

  /* All new struct kitties */
  const [kitties, setKitties] = useState([]);
  /* Kitties DNA */
  const [kittyDNAs, setKittyDNAs] = useState([]);
  /* Kitties Owners */
  const [kittyOwners, setKittyOwners] = useState([]);

  /* Get kitties storage info */
  const fetchKitties = () => {
    let unsubscribe;

    api.query.kittiesModule
      .kittyCount((cnt) => {
        if (cnt !== "") {
          /* Create array for all kitties id */
          const kittyIds = Array.from(Array(parseInt(cnt, 10)), (v, k) => k);

          /* Get owners of all kitties id */
          api.query.kittiesModule.kittyOwnedBy
            .multi(kittyIds, (kittyOwners) => {
              setKittyOwners(kittyOwners);
            })
            .catch(console.error);

          /* Get DNA of all kitties id */
          api.query.kittiesModule.kitties
            .multi(kittyIds, (kittyDna) => {
              setKittyDNAs(kittyDna);
            })
            .catch(console.error);
        }
      })
      .then((unsub) => {
        unsubscribe = unsub;
      })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  };

  /* Wrpa kitties info into new object */
  const wrapKitties = () => {
    /* Store all wraped kitties */
    const kitties = [];
    for (let i = 0; i < kittyDNAs.length; ++i) {
      const kitty = {};
      kitty.id = i;
      kitty.dna = kittyDNAs[i].unwrap();
      kitty.owner = keyring.encodeAddress(kittyOwners[i].unwrap());
      kitties[i] = kitty;
    }
    setKitties(kitties);
  };

  /* Effect hooks */
  useEffect(fetchKitties, [api, keyring]);
  useEffect(wrapKitties, [keyring, kittyDNAs, kittyOwners]);

  return (
    <Grid.Column width={16}>
      <h1>小毛孩</h1>
      <KittyCards kitties={kitties} setStatus={setStatus} />
      <Form style={{ margin: "1em 0" }}>
        <Form.Field style={{ textAlign: "center" }}>
          <TxButton
            label="创建小毛孩"
            type="SIGNED-TX"
            setStatus={setStatus}
            attrs={{
              palletRpc: "kittiesModule",
              callable: "create",
              inputParams: [],
              paramFields: [],
            }}
          />
        </Form.Field>
      </Form>
      <div style={{ overflowWrap: "break-word" }}>{status}</div>
    </Grid.Column>
  );
}
