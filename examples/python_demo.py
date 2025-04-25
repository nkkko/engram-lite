#!/usr/bin/env python3
"""
EngramAI Python Client Demo

This script demonstrates the basic functionality of the EngramAI Python client.
It shows how to:
- Connect to an EngramAI database
- Initialize vector search
- Add engrams and connections
- Perform vector and hybrid search
- Generate embeddings
"""

import os
import sys
import tempfile

try:
    import engram_lite
except ImportError:
    print("Error: engram_lite Python module not found.")
    print("Please build it with 'cargo build --features python' and ensure it's in your PYTHONPATH.")
    sys.exit(1)

def main():
    # Create a temporary directory for the database
    with tempfile.TemporaryDirectory() as db_dir:
        print(f"Creating EngramAI database in {db_dir}")
        
        # Initialize database
        db = engram_lite.PyEngramDB(db_dir)
        
        # Initialize vector search with default E5 model
        print("Initializing vector search with E5 model")
        db.init_vector_search(engram_lite.PyEmbeddingModelType.E5)
        
        # Get the embedding service
        embedding_service = db.get_embedding_service()
        print(f"Using embedding model: {embedding_service.get_model_name()}")
        print(f"Embedding dimensions: {embedding_service.get_dimensions()}")
        
        # Add some engrams
        print("\nAdding engrams...")
        
        engram1 = engram_lite.PyEngram(
            "Climate change is accelerating faster than predicted.",
            "research",
            0.9
        )
        engram1.metadata["category"] = "environment"
        
        engram2 = engram_lite.PyEngram(
            "Solar panels are becoming more affordable and efficient.",
            "observation",
            0.8
        )
        engram2.metadata["category"] = "technology"
        
        engram3 = engram_lite.PyEngram(
            "Renewable energy can replace fossil fuels for most applications.",
            "inference",
            0.7
        )
        engram3.metadata["category"] = "energy"
        
        # Add engrams to the database
        engram1_id = db.add_engram(engram1)
        engram2_id = db.add_engram(engram2)
        engram3_id = db.add_engram(engram3)
        
        print(f"Added engram: {engram1_id}")
        print(f"Added engram: {engram2_id}")
        print(f"Added engram: {engram3_id}")
        
        # Create connections between engrams
        print("\nCreating connections...")
        
        connection1 = engram_lite.PyConnection(
            engram1_id,
            engram3_id,
            "causes",
            0.8
        )
        
        connection2 = engram_lite.PyConnection(
            engram2_id,
            engram3_id,
            "supports",
            0.9
        )
        
        # Add connections to the database
        connection1_id = db.add_connection(connection1)
        connection2_id = db.add_connection(connection2)
        
        print(f"Added connection: {connection1_id}")
        print(f"Added connection: {connection2_id}")
        
        # Retrieve an engram
        print("\nRetrieving engram...")
        retrieved_engram = db.get_engram(engram1_id)
        
        if retrieved_engram:
            print(f"Retrieved engram: {retrieved_engram.content}")
            print(f"Source: {retrieved_engram.source}")
            print(f"Confidence: {retrieved_engram.confidence}")
            print(f"Metadata: {retrieved_engram.metadata}")
        
        # Perform a vector search
        print("\nPerforming vector search...")
        results = db.query_by_text("global warming and climate impacts", 2)
        
        print(f"Found {len(results)} results:")
        for i, result in enumerate(results):
            print(f"{i+1}. {result.content} (ID: {result.id})")
        
        # Generate embeddings directly
        print("\nGenerating embeddings...")
        
        embedding1 = embedding_service.embed_text("Climate change is a global crisis")
        embedding2 = embedding_service.embed_text("We need to reduce carbon emissions")
        
        print(f"Generated embedding with {embedding1.dimensions} dimensions")
        
        # Calculate similarity between embeddings
        similarity = embedding1.cosine_similarity(embedding2)
        print(f"Cosine similarity between embeddings: {similarity:.4f}")
        
        # Normalize an embedding
        embedding1.normalize()
        print("Normalized embedding")
        
        # Batch embed multiple texts
        print("\nBatch embedding multiple texts...")
        
        texts = [
            "Climate change affects biodiversity",
            "Renewable energy is the future",
            "Conservation efforts are important"
        ]
        
        batch_embeddings = embedding_service.embed_batch(texts)
        print(f"Generated {len(batch_embeddings)} embeddings in batch")
        
        print("\nDemo completed successfully!")

if __name__ == "__main__":
    main()