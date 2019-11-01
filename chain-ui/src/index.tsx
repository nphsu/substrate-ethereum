import Web3 from 'web3'
import { useCallback } from 'react'
import * as React from 'react'
import { render } from 'react-dom'
import { Form } from 'react-final-form'
import DAIBridgeJSON from '../contract/DAIBridge.json'
import { TransactionConfig } from 'web3-core'

function Root() {
    if (window.web3 !== undefined) {
        const web3 = new Web3(window.web3.currentProvider)
        const DAIBridgeABI: any = DAIBridgeJSON.abi
        const json: any = DAIBridgeJSON
        const DAIBridgeAddress: any = json.networks[web3.givenProvider.networkVersion].address
        const contract = new web3.eth.Contract(DAIBridgeABI, DAIBridgeAddress)
        console.log(contract)

        const handleSubmit = useCallback(async () => {
            try {
                const accounts = await window.ethereum.enable()
                const defaultAccount = accounts[0]
                console.log('defaultAccount', defaultAccount)
                const encodedData = contract.methods
                    .setTransfer()
                    .encodeABI()
                const count = await web3.eth.getTransactionCount(defaultAccount)
                const tx: TransactionConfig = {
                    from: defaultAccount,
                    to: DAIBridgeAddress,
                    gasPrice: web3.utils.toHex(3 * 1e10),
                    gas: web3.utils.toHex(5000000),
                    nonce: +('0x' + count.toString(16)),
                    data: encodedData,
                    chainId: 42
                }
                web3.eth.sendTransaction(tx, (e: Error, id: string) => {
                    console.log(e)
                })
            } catch (error) {
                console.log(error)
            }
        }, [])
        return (
            <Form
                onSubmit={handleSubmit}
                subscription={{ submitting: true, submitError: true }}>
                {({ handleSubmit, submitting, submitError }): React.ReactElement<{}> => (
                    <form onSubmit={handleSubmit}>
                        <button type="submit">
                            Call setTransfer()
                    </button>
                    </form>
                )}
            </Form>

        )
    } else {
        return <div>You need to install Metamask</div>;
    }
}
const root = document.getElementById("root")
render(<Root />, root)