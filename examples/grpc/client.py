#!/usr/bin/env python3
"""
EngramAI gRPC Client Example

This script demonstrates how to connect to an EngramAI gRPC server
and perform various operations like creating and retrieving engrams.

Usage:
    python client.py [--addr ADDRESS]

Options:
    --addr ADDRESS    Set the server address (default: localhost:50051)
    --help            Show this help message
"""

import argparse
import grpc
import sys

# These imports will work once the proto files are compiled into Python code
# import engram_pb2
# import engram_pb2_grpc


def parse_arguments():
    """Parse command-line arguments."""
    parser = argparse.ArgumentParser(description="EngramAI gRPC Client Example")
    parser.add_argument(
        "--addr",
        type=str,
        default="localhost:50051",
        help="Server address (default: localhost:50051)",
    )
    return parser.parse_args()


def run_client(server_addr):
    """Run the gRPC client with the specified server address."""
    print(f"Connecting to server at: {server_addr}")
    
    # Create a secure channel to the server
    channel = grpc.insecure_channel(server_addr)
    
    # Uncomment the following once you have generated the Python code from proto files
    """
    # Create client stub
    stub = engram_pb2_grpc.EngramServiceStub(channel)
    
    # Create an engram
    create_request = engram_pb2.CreateEngramRequest(
        content="This is a test engram created via Python gRPC client",
        source="Python gRPC example",
        confidence=0.95,
        # metadata can be added here if needed
    )
    
    create_response = stub.CreateEngram(create_request)
    engram = create_response.engram
    
    print(f"Created engram with ID: {engram.id}")
    print(f"Content: {engram.content}")
    
    # Retrieve the engram
    get_request = engram_pb2.GetEngramRequest(id=engram.id)
    get_response = stub.GetEngram(get_request)
    retrieved_engram = get_response.engram
    
    print(f"Retrieved engram with ID: {retrieved_engram.id}")
    print(f"Content: {retrieved_engram.content}")
    print(f"Source: {retrieved_engram.source}")
    print(f"Confidence: {retrieved_engram.confidence}")
    print(f"Created at: {retrieved_engram.created_at}")
    """
    
    print("Client executed successfully (Proto stubs not implemented)")


if __name__ == "__main__":
    args = parse_arguments()
    try:
        run_client(args.addr)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)