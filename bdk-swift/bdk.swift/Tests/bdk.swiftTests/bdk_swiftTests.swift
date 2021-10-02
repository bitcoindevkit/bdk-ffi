import XCTest
import Clibbdkffi

@testable import bdk_swift

final class bdk_swiftTests: XCTestCase {
    func testExample() throws {
        // This is an example of a functional test case.
        // Use XCTAssert and related functions to verify your tests produce the correct
        // results.
        XCTAssertEqual(bdk_swift().text, "Hello, World!")
        let desc = "wpkh([bf988dd3/84'/1'/0']tpubDD7bHVspyCSvvU8qEycydF664NAX6EAPjJ77j9E614GU2zVdXgnZZo6JJjKbDT6fUn8owMN6TCP9rZMznsNEhJbpkEwp6fAyyoSqy3DH2Qj/0/*)";
        let change = "wpkh([bf988dd3/84'/1'/0']tpubDD7bHVspyCSvvU8qEycydF664NAX6EAPjJ77j9E614GU2zVdXgnZZo6JJjKbDT6fUn8owMN6TCP9rZMznsNEhJbpkEwp6fAyyoSqy3DH2Qj/1/*)";
        let net = "testnet";
        let blocks = "ssl://electrum.blockstream.info:60002";
            
        let bc_config = new_electrum_config(blocks, nil, 5, 30, 100)
        let db_config = new_memory_config()
        
        let wallet_result = new_wallet_result(desc,change,net,bc_config,db_config)
        
        free_blockchain_config(bc_config)
        free_database_config(db_config)       
        
        let wallet = wallet_result.ok
        let sync_result = sync_wallet(wallet)   
        assert(sync_result.err == FFI_ERROR_NONE) 
        free_void_result(sync_result) 
        
        let address1_result = new_address(wallet).ok
        let address1 = String(cString: address1_result!, encoding: .utf8)
        //print("address1 = \(address1!)")
        assert(address1! == "tb1qh4ajvhz9nd76tqddnl99l89hx4dat33hrjauzw")
    }
}
