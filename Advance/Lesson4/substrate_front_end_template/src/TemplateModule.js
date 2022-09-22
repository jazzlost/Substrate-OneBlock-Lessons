import React, { useEffect, useState } from 'react'
import { Form, Input, Grid, Card, Statistic, Button } from 'semantic-ui-react'

import { useSubstrateState } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

function str2UTF8(str) {
  var bytes = [];
  var len, c;
  len = str.length;
  for (var i = 0; i < len; i++) {
    c = str.charCodeAt(i);
    if (c >= 0x010000 && c <= 0x10FFFF) {
      bytes.push(((c >> 18) & 0x07) | 0xF0);
      bytes.push(((c >> 12) & 0x3F) | 0x80);
      bytes.push(((c >> 6) & 0x3F) | 0x80);
      bytes.push((c & 0x3F) | 0x80);
    } else if (c >= 0x000800 && c <= 0x00FFFF) {
      bytes.push(((c >> 12) & 0x0F) | 0xE0);
      bytes.push(((c >> 6) & 0x3F) | 0x80);
      bytes.push((c & 0x3F) | 0x80);
    } else if (c >= 0x000080 && c <= 0x0007FF) {
      bytes.push(((c >> 6) & 0x1F) | 0xC0);
      bytes.push((c & 0x3F) | 0x80);
    } else {
      bytes.push(c & 0xFF);
    }
  }
  return bytes;
}

function byteToString(arr) {
  if (typeof arr === 'string') {
    return arr;
  }
  var str = '',
    _arr = arr;
  for (var i = 0; i < _arr.length; i++) {
    var one = _arr[i].toString(2),
      v = one.match(/^1+?(?=0)/);
    if (v && one.length == 8) {
      var bytesLength = v[0].length;
      var store = _arr[i].toString(2).slice(7 - bytesLength);
      for (var st = 1; st < bytesLength; st++) {
        store += _arr[st + i].toString(2).slice(2);
      }
      str += String.fromCharCode(parseInt(store, 2));
      i += bytesLength - 1;
    } else {
      str += String.fromCharCode(_arr[i]);
    }
  }
  return str;
}


function BytesToIntLittleEndian(bytes) {
  var val = 0;
  for (var i = bytes.length - 1; i >= 0; i--) {
    val += bytes[i];
    if (i != 0) {
      val = val << 8;
    }
  }
  return val;
}


function IntToBytesLittleEndian(number, length) {
  var bytes = [];
  var i = 0;
  do {
    bytes[i++] = number & (255);
    number = number >> 8;
  } while (i < length)
  return bytes;
}


function Main(props) {
  const { api } = useSubstrateState()

  // The transaction submission status
  const [status, setStatus] = useState('')

  // The currently stored value
  const [currentValue, setCurrentValue] = useState(0)
  const [formValue, setFormValue] = useState(0)
  
  // Offchain data value
  const [offChainValue, setOffChainValue] = useState(0)
  // Block number to query
  const [queryBlock, setQueryBlock] = useState(0)
  // Query state
  const [canQurty, setCanQuery] = useState(false)

  useEffect(() => {
    let unsubscribe
    api.query.templateModule
      .something(newValue => {
        // The storage value is an Option<u32>
        // So we have to check whether it is None first
        // There is also unwrapOr
        if (newValue.isNone) {
          setCurrentValue('<None>')
        } else {
          setCurrentValue(newValue.unwrap().toNumber())
        }
      })
      .then(unsub => {
        unsubscribe = unsub
      })
      .catch(console.error)

    return () => unsubscribe && unsubscribe()
  }, [api.query.templateModule])

  useEffect(() => {
    const getLocalStorage = async () => {
      // Convert block number to bytes array
      const block_number_bytes = IntToBytesLittleEndian(queryBlock, 4)
      console.log("block_number_bytes " + block_number_bytes)
      // Make storage key
      const key = str2UTF8("node-template::indexing::").concat(block_number_bytes);
      // Get local storage by key
      const data = await api.rpc.offchain.localStorageGet('PERSISTENT', key)
      
      // Text part of storage 
      const text = data.value.slice(1, 20)
      const rawData = byteToString(text)
      
      // Number part of storage
      const num = data.value.slice(20, 24)
      const rawNum = BytesToIntLittleEndian(num)
      
      console.log({ data, key, text, rawNum, rawData })

      setOffChainValue(rawData + ", " + rawNum)
    }

    // Only press button can active query state
    if (canQurty) {
      getLocalStorage()
      setCanQuery(false)
    }
  }, [canQurty])

  return (
    <Grid.Column width={8}>
      <h1>Template Module</h1>
      <Card centered>
        <Card.Content textAlign="center">
          <Statistic label="Current Value" value={currentValue} />
        </Card.Content>
      </Card>
      <Form>
        <Form.Field>
          <Input
            label="New Value"
            state="newValue"
            type="number"
            onChange={(_, { value }) => setFormValue(value)}
          />
        </Form.Field>
        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            label="Store Something"
            type="SIGNED-TX"
            setStatus={setStatus}
            attrs={{
              palletRpc: 'templateModule',
              callable: 'doSomething',
              inputParams: [formValue],
              paramFields: [true],
            }}
          />
        </Form.Field>
        <div style={{ overflowWrap: 'break-word' }}>{status}</div>
      </Form>
      <h1>Offchain Storage</h1>
      <Card centered>
        <Card.Content textAlign="center">
          <Statistic label="Offchain Value" value={offChainValue} />
        </Card.Content>
      </Card>
      <Form>
        <Form.Field>
          <Input
            label="Block Number"
            state="newValue"
            type="number"
            onChange={(_, { value }) => setQueryBlock(value)}
          />
        </Form.Field>
        <Form.Field style={{ textAlign: 'center' }}>
          <Button
            basic
            label="Query Storage"
            type="submit"
            onClick={() => setCanQuery(true)}
          />
        </Form.Field>
        <div style={{ overflowWrap: 'break-word' }}>{status}</div>
      </Form>
    </Grid.Column>
  )
}

export default function TemplateModule(props) {
  const { api } = useSubstrateState()
  return api.query.templateModule && api.query.templateModule.something ? (
    <Main {...props} />
  ) : null
}
