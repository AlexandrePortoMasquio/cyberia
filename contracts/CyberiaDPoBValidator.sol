// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// Importação das dependências da OpenZeppelin
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract CyberiaToken is ERC20, Ownable {

    // Fornecimento inicial de 1 bilhão de tokens, com 18 casas decimais padrão.
    uint256 public constant INITIAL_SUPPLY = 1_000_000_000 * (10 ** 18);

    // Construtor do contrato
    constructor() ERC20("Cyberia Token", "XCYB") {
        // Mint inicial do total supply para sua conta
        _mint(msg.sender, INITIAL_SUPPLY);
    }
}
